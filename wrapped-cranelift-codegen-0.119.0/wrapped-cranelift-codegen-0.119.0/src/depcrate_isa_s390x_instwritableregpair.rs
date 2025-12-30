// Generated macro for WritableRegPair (struct)
macro_rules! Depcrate_isa_s390x_instWritableRegPair {
() => {
// Module: crate::isa::s390x::inst
// Provides: {"WritableRegPair"}
// Dependencies: {}
# [doc = " A writable register pair. Enum so it can be destructured in ISLE."] # [derive (Clone , Copy , Debug)] pub struct WritableRegPair { pub hi : Writable < Reg > , pub lo : Writable < Reg > , }
};
}
