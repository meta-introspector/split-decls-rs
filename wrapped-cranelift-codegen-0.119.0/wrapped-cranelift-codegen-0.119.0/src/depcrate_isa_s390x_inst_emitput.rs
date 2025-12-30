// Generated macro for put (function)
macro_rules! Depcrate_isa_s390x_inst_emitput {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"put"}
// Dependencies: {}
# [doc = " Emit encoding to sink."] fn put (sink : & mut MachBuffer < Inst > , enc : & [u8]) { for byte in enc { sink . put1 (* byte) ; } }
};
}
