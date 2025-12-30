// Generated macro for PulleyCall (struct)
macro_rules! Depcrate_isa_pulley_shared_inst_argsPulleyCall {
() => {
// Module: crate::isa::pulley_shared::inst::args
// Provides: {"PulleyCall"}
// Dependencies: {}
# [doc = " Payload of `CallInfo` for call instructions"] # [derive (Clone , Debug)] pub struct PulleyCall { # [doc = " The external name that's being called, or the Cranelift-generated"] # [doc = " function that's being invoked."] pub name : ExternalName , # [doc = " Arguments tracked in this call invocation which aren't assigned fixed"] # [doc = " registers. This tracks up to 4 registers and all remaining registers"] # [doc = " will be present and tracked in `CallInfo<T>` fields."] pub args : SmallVec < [XReg ; 4] > , }
};
}
