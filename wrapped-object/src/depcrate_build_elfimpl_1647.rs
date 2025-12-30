// Generated macro for impl_1647 (impl)
macro_rules! Depcrate_build_elfimpl_1647 {
() => {
// Module: crate::build::elf
// Provides: {"impl_1647"}
// Dependencies: {}
impl < 'data > VersionDef < 'data > { # [doc = " Optimise for the common case where the first version is the same as the base version."] fn is_shared (& self , index : usize , base : Option < & ByteString < '_ > >) -> bool { index == 1 && self . names . len () == 1 && self . names . first () == base } }
};
}
