// Generated macro for impl_109 (impl)
macro_rules! Depcrate_defimpl_109 {
() => {
// Module: crate::def
// Provides: {"impl_109"}
// Dependencies: {}
impl PartialRes { # [inline] pub fn new (base_res : Res < NodeId >) -> Self { PartialRes { base_res , unresolved_segments : 0 } } # [inline] pub fn with_unresolved_segments (base_res : Res < NodeId > , mut unresolved_segments : usize) -> Self { if base_res == Res :: Err { unresolved_segments = 0 } PartialRes { base_res , unresolved_segments } } # [inline] pub fn base_res (& self) -> Res < NodeId > { self . base_res } # [inline] pub fn unresolved_segments (& self) -> usize { self . unresolved_segments } # [inline] pub fn full_res (& self) -> Option < Res < NodeId > > { (self . unresolved_segments == 0) . then_some (self . base_res) } # [inline] pub fn expect_full_res (& self) -> Res < NodeId > { self . full_res () . expect ("unexpected unresolved segments") } }
};
}
