// Generated macro for impl_482 (impl)
macro_rules! Depcrate_options_configimpl_482 {
() => {
// Module: crate::options::config
// Provides: {"impl_482"}
// Dependencies: {}
impl FromOverride < FileKindsOverride > for FileKinds { fn from (value : FileKindsOverride , default : Self) -> Self { FileKinds { normal : FromOverride :: from (value . normal , default . normal) , directory : FromOverride :: from (value . directory , default . directory) , symlink : FromOverride :: from (value . symlink , default . symlink) , pipe : FromOverride :: from (value . pipe , default . pipe) , block_device : FromOverride :: from (value . block_device , default . block_device) , char_device : FromOverride :: from (value . char_device , default . char_device) , socket : FromOverride :: from (value . socket , default . socket) , special : FromOverride :: from (value . special , default . special) , executable : FromOverride :: from (value . executable , default . executable) , mount_point : FromOverride :: from (value . mount_point , default . mount_point) , } } }
};
}
