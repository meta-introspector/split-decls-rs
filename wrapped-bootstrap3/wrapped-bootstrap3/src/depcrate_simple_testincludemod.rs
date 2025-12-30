// Generated macro for includemod (macro)
macro_rules! Depcrate_simple_testincludemod {
() => {
// Module: crate::simple_test
// Provides: {"includemod"}
// Dependencies: {}
macro_rules ! includemod { ($ mod_name : ident , $ file_path : expr) => { pub mod $ mod_name { use std :: path :: { Path , PathBuf } ; use anyhow :: Result ; use crate :: split_decls_config_mod :: SplitDeclsConfig ; include ! ($ file_path) ; } } ; }
};
}
