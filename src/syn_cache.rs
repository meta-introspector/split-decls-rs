use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCache {
    pub content: String,
    pub modified_time: u64,
    pub parsed_ast: Option<String>, // Serialized AST
}

#[derive(Debug, Default)]
pub struct SynCache {
    cache: HashMap<String, FileCache>,
    cache_file: String,
}

impl SynCache {
    pub fn new(cache_file: &str) -> Self {
        let mut cache = Self {
            cache: HashMap::new(),
            cache_file: cache_file.to_string(),
        };
        let _ = cache.load();
        cache
    }

    pub fn get_cached_content(&mut self, path: &Path) -> Result<String> {
        let path_str = path.to_string_lossy().to_string();
        
        // Get file modification time
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?.duration_since(SystemTime::UNIX_EPOCH)?.as_secs();
        
        // Check cache
        if let Some(cached) = self.cache.get(&path_str) {
            if cached.modified_time == modified {
                return Ok(cached.content.clone());
            }
        }
        
        // Read and cache
        let content = fs::read_to_string(path)?;
        self.cache.insert(path_str, FileCache {
            content: content.clone(),
            modified_time: modified,
            parsed_ast: None,
        });
        
        Ok(content)
    }

    pub fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.cache)?;
        fs::write(&self.cache_file, json)?;
        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        if let Ok(content) = fs::read_to_string(&self.cache_file) {
            self.cache = serde_json::from_str(&content)?;
        }
        Ok(())
    }
}
