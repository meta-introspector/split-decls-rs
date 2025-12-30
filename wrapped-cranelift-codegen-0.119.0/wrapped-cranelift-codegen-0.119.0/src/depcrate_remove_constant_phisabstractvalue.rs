// Generated macro for AbstractValue (enum)
macro_rules! Depcrate_remove_constant_phisAbstractValue {
() => {
// Module: crate::remove_constant_phis
// Provides: {"AbstractValue"}
// Dependencies: {}
# [doc = " The `Value`s (Group B) that can flow to a formal parameter (Group A)."] # [derive (Clone , Copy , Debug , PartialEq)] enum AbstractValue { # [doc = " Two or more values flow to this formal."] Many , # [doc = " Exactly one value, as stated, flows to this formal.  The `Value`s that"] # [doc = " can appear here are exactly: `Value`s defined by `Inst`s, plus the"] # [doc = " `Value`s defined by the formals of the entry block.  Note that this is"] # [doc = " exactly the set of `Value`s that are *not* tracked in the solver below"] # [doc = " (see `SolverState`)."] One (Value) , # [doc = " No value flows to this formal."] None , }
};
}
