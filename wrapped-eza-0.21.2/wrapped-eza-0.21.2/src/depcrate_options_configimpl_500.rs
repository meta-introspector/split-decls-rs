// Generated macro for impl_500 (impl)
macro_rules! Depcrate_options_configimpl_500 {
() => {
// Module: crate::options::config
// Provides: {"impl_500"}
// Dependencies: {}
impl FromOverride < FileTypeOverride > for FileType { fn from (value : FileTypeOverride , default : Self) -> Self { FileType { image : FromOverride :: from (value . image , default . image) , video : FromOverride :: from (value . video , default . video) , music : FromOverride :: from (value . music , default . music) , lossless : FromOverride :: from (value . lossless , default . lossless) , crypto : FromOverride :: from (value . crypto , default . crypto) , document : FromOverride :: from (value . document , default . document) , compressed : FromOverride :: from (value . compressed , default . compressed) , temp : FromOverride :: from (value . temp , default . temp) , compiled : FromOverride :: from (value . compiled , default . compiled) , build : FromOverride :: from (value . build , default . build) , source : FromOverride :: from (value . source , default . source) , } } }
};
}
