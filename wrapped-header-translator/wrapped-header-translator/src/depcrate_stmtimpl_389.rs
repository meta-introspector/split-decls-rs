// Generated macro for impl_389 (impl)
macro_rules! Depcrate_stmtimpl_389 {
() => {
// Module: crate::stmt
// Provides: {"impl_389"}
// Dependencies: {}
impl Abi { fn extern_inner (& self) -> & 'static str { match self { Self :: OuterRustInnerC | Self :: C => "extern \"C\" " , Self :: OuterRustInnerCUnwind | Self :: CUnwind => "extern \"C-unwind\" " , } } fn extern_outer (& self) -> & 'static str { if self . rust_outer () { "" } else { self . extern_inner () } } fn rust_outer (& self) -> bool { matches ! (self , Self :: OuterRustInnerC | Self :: OuterRustInnerCUnwind) } pub (crate) fn as_rust_outer (& self) -> Self { match self { Self :: C => Self :: OuterRustInnerC , Self :: CUnwind => Self :: OuterRustInnerCUnwind , Self :: OuterRustInnerC | Self :: OuterRustInnerCUnwind => panic ! ("already Rust outer") , } } }
};
}
