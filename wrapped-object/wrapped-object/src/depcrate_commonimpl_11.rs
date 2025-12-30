// Generated macro for impl_11 (impl)
macro_rules! Depcrate_commonimpl_11 {
() => {
// Module: crate::common
// Provides: {"impl_11"}
// Dependencies: {}
impl BinaryFormat { # [doc = " The target's native binary format for relocatable object files."] # [doc = ""] # [doc = " Defaults to `Elf` for unknown platforms."] pub fn native_object () -> BinaryFormat { if cfg ! (target_os = "windows") { BinaryFormat :: Coff } else if cfg ! (target_os = "macos") { BinaryFormat :: MachO } else { BinaryFormat :: Elf } } }
};
}
