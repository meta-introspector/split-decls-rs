use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileCache {
    pub content: String,
    pub git_hash: Option<String>,
    pub last_modified: u64,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BootstrapCache {
    files: HashMap<PathBuf, FileCache>,
    git_tree_hash: Option<String>,
}

impl BootstrapCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_file(&mut self, path: &Path) -> Result<String> {
        if let Some(cached) = self.files.get(path) {
            return Ok(cached.content.clone());
        }

        let content = fs::read_to_string(path)?;
        let git_hash = self.get_git_hash(path);
        let metadata = fs::metadata(path)?;
        let last_modified = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs();

        let cache_entry = FileCache {
            content: content.clone(),
            git_hash,
            last_modified,
        };

        self.files.insert(path.to_path_buf(), cache_entry);
        Ok(content)
    }

    fn get_git_hash(&self, path: &Path) -> Option<String> {
        std::process::Command::new("git")
            .args(&["hash-object", path.to_str()?])
            .output()
            .ok()
            .and_then(|output| {
                if output.status.success() {
                    String::from_utf8(output.stdout).ok().map(|s| s.trim().to_string())
                } else {
                    None
                }
            })
    }

    pub fn save_cache(&self, cache_path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(cache_path, json)?;
        Ok(())
    }

    pub fn load_cache(cache_path: &Path) -> Result<Self> {
        if cache_path.exists() {
            let json = fs::read_to_string(cache_path)?;
            Ok(serde_json::from_str(&json)?)
        } else {
            Ok(Self::new())
        }
    }
}
