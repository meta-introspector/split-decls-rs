// Generated macro for impl_470 (impl)
macro_rules! Depcrate_options_configimpl_470 {
() => {
// Module: crate::options::config
// Provides: {"impl_470"}
// Dependencies: {}
impl < S , T > FromOverride < Option < S > > for Option < T > where T : FromOverride < S > + Default , { fn from (value : Option < S > , default : Option < T >) -> Option < T > { match (value , default) { (Some (value) , Some (default)) => Some (FromOverride :: from (value , default)) , (Some (value) , None) => Some (FromOverride :: from (value , T :: default ())) , (None , Some (default)) => Some (default) , (None , None) => None , } } }
};
}
