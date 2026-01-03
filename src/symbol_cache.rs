use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use shared_memory::*;
use std::io::Read;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedSymbol {
    pub name: String,
    pub symbol_type: String,
    pub source_file: String,
    pub dependencies: Vec<String>,
}

pub struct SymbolCache {
    shmem: Option<Shmem>,
    symbols: HashMap<String, CachedSymbol>,
}

impl SymbolCache {
    pub fn new() -> Self {
        Self {
            shmem: None,
            symbols: HashMap::new(),
        }
    }

    pub fn load_or_create(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Try to open existing shared memory
        match ShmemConf::new().size(64 * 1024 * 1024).flink("rustc_symbols").open() {
            Ok(shmem) => {
                println!("📋 Found existing symbol cache in shared memory");
                self.shmem = Some(shmem);
                self.load_from_shmem()?;
            }
            Err(_) => {
                println!("🔄 Creating new symbol cache...");
                self.load_from_file()?;
                self.create_shmem()?;
            }
        }
        Ok(())
    }

    fn load_from_shmem(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref shmem) = self.shmem {
            let data = unsafe { std::slice::from_raw_parts(shmem.as_ptr(), shmem.len()) };
            
            // Find the end of valid data (look for null terminator)
            let end = data.iter().position(|&b| b == 0).unwrap_or(data.len());
            let json_data = &data[..end];
            
            if !json_data.is_empty() {
                self.symbols = serde_json::from_slice(json_data)?;
                println!("✅ Loaded {} symbols from shared memory", self.symbols.len());
            }
        }
        Ok(())
    }

    fn load_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !std::path::Path::new("symbol_map.json.gz").exists() {
            return Ok(());
        }

        let file = std::fs::File::open("symbol_map.json.gz")?;
        let mut decoder = flate2::read::GzDecoder::new(file);
        let mut contents = String::new();
        decoder.read_to_string(&mut contents)?;

        let raw_symbols: HashMap<String, serde_json::Value> = serde_json::from_str(&contents)?;
        
        for (name, value) in raw_symbols.iter().take(1000) { // Limit for testing
            if let Ok(symbol) = serde_json::from_value::<CachedSymbol>(value.clone()) {
                self.symbols.insert(name.clone(), symbol);
            }
        }

        println!("✅ Loaded {} symbols from file", self.symbols.len());
        Ok(())
    }

    fn create_shmem(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_vec(&self.symbols)?;
        
        let shmem = ShmemConf::new()
            .size(64 * 1024 * 1024)
            .flink("rustc_symbols")
            .create()?;

        unsafe {
            let ptr = shmem.as_ptr() as *mut u8;
            let len = json_data.len().min(shmem.len() - 1);
            std::ptr::copy_nonoverlapping(json_data.as_ptr(), ptr, len);
            *ptr.add(len) = 0; // Null terminator
        }

        self.shmem = Some(shmem);
        println!("💾 Created shared memory cache ({} bytes)", json_data.len());
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&CachedSymbol> {
        self.symbols.get(name)
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.symbols.keys()
    }

    pub fn len(&self) -> usize {
        self.symbols.len()
    }
}
