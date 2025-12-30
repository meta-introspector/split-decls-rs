// Generated macro for put_with_trap (function)
macro_rules! Depcrate_isa_s390x_inst_emitput_with_trap {
() => {
// Module: crate::isa::s390x::inst::emit
// Provides: {"put_with_trap"}
// Dependencies: {}
# [doc = " Emit encoding to sink, adding a trap on the last byte."] fn put_with_trap (sink : & mut MachBuffer < Inst > , enc : & [u8] , trap_code : TrapCode) { let len = enc . len () ; for i in 0 .. len - 1 { sink . put1 (enc [i]) ; } sink . add_trap (trap_code) ; sink . put1 (enc [len - 1]) ; }
};
}
