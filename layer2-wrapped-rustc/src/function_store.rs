use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tokio::time::{sleep, Duration};
use notify::{Watcher, RecursiveMode, RecommendedWatcher, Config};
use std::sync::mpsc::channel;

pub struct FunctionStore {
    functions_dir: String,
    loaded_functions: HashMap<String, String>,
}

impl FunctionStore {
    pub fn new(functions_dir: &str) -> Self {
        fs::create_dir_all(functions_dir).ok();
        Self {
            functions_dir: functions_dir.to_string(),
            loaded_functions: HashMap::new(),
        }
    }

    pub fn save_function(&mut self, name: &str, code: &str) -> Result<(), Box<dyn std::error::Error>> {
        let safe_name = name.replace("::", "_").replace("<", "_").replace(">", "_");
        let file_path = format!("{}/{}.rs", self.functions_dir, safe_name);
        
        fs::write(&file_path, code)?;
        self.loaded_functions.insert(name.to_string(), code.to_string());
        
        println!("💾 Saved function '{}' to {}", name, file_path);
        Ok(())
    }

    pub fn load_function(&mut self, name: &str) -> Option<String> {
        let safe_name = name.replace("::", "_").replace("<", "_").replace(">", "_");
        let file_path = format!("{}/{}.rs", self.functions_dir, safe_name);
        
        if let Ok(code) = fs::read_to_string(&file_path) {
            self.loaded_functions.insert(name.to_string(), code.clone());
            Some(code)
        } else {
            None
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&String> {
        self.loaded_functions.get(name)
    }

    pub async fn watch_for_changes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        watcher.watch(Path::new(&self.functions_dir), RecursiveMode::Recursive)?;

        println!("👁️  Watching {} for function changes...", self.functions_dir);

        loop {
            match rx.try_recv() {
                Ok(event) => {
                    if let Ok(event) = event {
                        if let Some(path) = event.paths.first() {
                            if path.extension().map_or(false, |ext| ext == "rs") {
                                self.reload_from_file(path).await?;
                            }
                        }
                    }
                }
                Err(_) => {
                    sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }

    async fn reload_from_file(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(stem) = path.file_stem() {
            let safe_name = stem.to_string_lossy();
            let function_name = safe_name.replace("_", "::");
            
            if let Ok(code) = fs::read_to_string(path) {
                self.loaded_functions.insert(function_name.clone(), code);
                println!("🔄 Reloaded function '{}' from {:?}", function_name, path);
            }
        }
        Ok(())
    }

    pub fn list_functions(&self) -> Vec<String> {
        self.loaded_functions.keys().cloned().collect()
    }
}
