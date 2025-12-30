// Generated macro for impl_2068 (impl)
macro_rules! Depcrate_isa_riscv64_inst_argsimpl_2068 {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"impl_2068"}
// Dependencies: {}
impl Inst { # [doc = " fence request bits."] pub (crate) const FENCE_REQ_I : u8 = 1 << 3 ; pub (crate) const FENCE_REQ_O : u8 = 1 << 2 ; pub (crate) const FENCE_REQ_R : u8 = 1 << 1 ; pub (crate) const FENCE_REQ_W : u8 = 1 << 0 ; pub (crate) fn fence_req_to_string (x : u8) -> String { let mut s = String :: default () ; if x & Self :: FENCE_REQ_I != 0 { s . push_str ("i") ; } if x & Self :: FENCE_REQ_O != 0 { s . push_str ("o") ; } if x & Self :: FENCE_REQ_R != 0 { s . push_str ("r") ; } if x & Self :: FENCE_REQ_W != 0 { s . push_str ("w") ; } s } }
};
}
