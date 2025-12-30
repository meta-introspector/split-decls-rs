// Generated macro for NewOrExistingInst (enum)
macro_rules! Depcrate_egraphNewOrExistingInst {
() => {
// Module: crate::egraph
// Provides: {"NewOrExistingInst"}
// Dependencies: {}
# [doc = " For passing to `insert_pure_enode`. Sometimes the enode already"] # [doc = " exists as an Inst (from the original CLIF), and sometimes we're in"] # [doc = " the middle of creating it and want to avoid inserting it if"] # [doc = " possible until we know we need it."] pub (crate) enum NewOrExistingInst { New (InstructionData , Type) , Existing (Inst) , }
};
}
