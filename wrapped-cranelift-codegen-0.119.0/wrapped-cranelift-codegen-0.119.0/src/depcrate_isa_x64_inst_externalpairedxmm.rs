// Generated macro for PairedXmm (struct)
macro_rules! Depcrate_isa_x64_inst_externalPairedXmm {
() => {
// Module: crate::isa::x64::inst::external
// Provides: {"PairedXmm"}
// Dependencies: {}
# [doc = " A pair of XMM registers, one for reading and one for writing."] # [derive (Clone , Copy , Debug)] pub struct PairedXmm { pub (crate) read : Xmm , pub (crate) write : WritableXmm , }
};
}
