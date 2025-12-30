// Generated macro for impl_1203 (impl)
macro_rules! Depcrate_core_configimpl_1203 {
() => {
// Module: crate::core::config
// Provides: {"impl_1203"}
// Dependencies: {}
impl < T > Merge for Option < T > { fn merge (& mut self , _parent_config_path : Option < PathBuf > , _included_extensions : & mut HashSet < PathBuf > , other : Self , replace : ReplaceOpt ,) { match replace { ReplaceOpt :: IgnoreDuplicate => { if self . is_none () { * self = other ; } } ReplaceOpt :: Override => { if other . is_some () { * self = other ; } } ReplaceOpt :: ErrorOnDuplicate => { if other . is_some () { if self . is_some () { if cfg ! (test) { panic ! ("overriding existing option") } else { eprintln ! ("overriding existing option") ; exit ! (2) ; } } else { * self = other ; } } } } } }
};
}
