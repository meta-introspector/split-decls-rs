use split_decls_types::SplitDeclsConfig;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref GLOBAL_CONFIG: Mutex<SplitDeclsConfig> = Mutex::new(SplitDeclsConfig::default());
}

#[macro_export]
macro_rules! load_config {
    ($path:expr) => {
        let content = std::fs::read_to_string($path).unwrap();
        let config: split_decls_types::SplitDeclsConfig = toml::from_str(&content).unwrap();
        *crate::config_macros::GLOBAL_CONFIG.lock().unwrap() = config;
    };
}

#[macro_export]
macro_rules! mkwrapping {
    () => {
        crate::config_macros::GLOBAL_CONFIG.lock().unwrap().wrapping.clone()
    };
}
