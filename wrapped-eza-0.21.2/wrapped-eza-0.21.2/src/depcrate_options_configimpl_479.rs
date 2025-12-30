// Generated macro for impl_479 (impl)
macro_rules! Depcrate_options_configimpl_479 {
() => {
// Module: crate::options::config
// Provides: {"impl_479"}
// Dependencies: {}
impl FromOverride < FileNameStyleOverride > for FileNameStyle { fn from (value : FileNameStyleOverride , default : Self) -> Self { FileNameStyle { icon : FromOverride :: from (value . icon , default . icon) , filename : FromOverride :: from (value . filename , default . filename) , } } }
};
}
