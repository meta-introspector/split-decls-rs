// Generated macro for impl_145 (impl)
macro_rules! Depcrateimpl_145 {
() => {
// Module: crate
// Provides: {"impl_145"}
// Dependencies: {}
impl AsmFileExt { fn from_path (file : & Path) -> Option < Self > { if let Some (ext) = file . extension () { if let Some (ext) = ext . to_str () { let ext = ext . to_lowercase () ; match & * ext { "asm" => return Some (AsmFileExt :: DotAsm) , "s" => return Some (AsmFileExt :: DotS) , _ => return None , } } } None } }
};
}
