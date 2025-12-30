// Generated macro for impl_490 (impl)
macro_rules! Depcrate_options_configimpl_490 {
() => {
// Module: crate::options::config
// Provides: {"impl_490"}
// Dependencies: {}
impl FromOverride < LinksOverride > for Links { fn from (value : LinksOverride , default : Self) -> Self { Links { normal : FromOverride :: from (value . normal , default . normal) , multi_link_file : FromOverride :: from (value . multi_link_file , default . multi_link_file) , } } }
};
}
