// Generated macro for Checker (struct)
macro_rules! Depcrate_check_consts_checkChecker {
() => {
// Module: crate::check_consts::check
// Provides: {"Checker"}
// Dependencies: {}
pub struct Checker < 'mir , 'tcx > { ccx : & 'mir ConstCx < 'mir , 'tcx > , qualifs : Qualifs < 'mir , 'tcx > , # [doc = " The span of the current statement."] span : Span , # [doc = " A set that stores for each local whether it is \"transient\", i.e. guaranteed to be dead"] # [doc = " when this MIR body returns."] transient_locals : Option < DenseBitSet < Local > > , error_emitted : Option < ErrorGuaranteed > , secondary_errors : Vec < Diag < 'tcx > > , }
};
}
