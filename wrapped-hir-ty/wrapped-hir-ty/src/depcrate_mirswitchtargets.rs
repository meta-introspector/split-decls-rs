// Generated macro for SwitchTargets (struct)
macro_rules! Depcrate_mirSwitchTargets {
() => {
// Module: crate::mir
// Provides: {"SwitchTargets"}
// Dependencies: {}
# [derive (Debug , Clone , Hash , PartialEq , Eq)] pub struct SwitchTargets < 'db > { # [doc = " Possible values. The locations to branch to in each case"] # [doc = " are found in the corresponding indices from the `targets` vector."] values : SmallVec < [u128 ; 1] > , # [doc = " Possible branch sites. The last element of this vector is used"] # [doc = " for the otherwise branch, so targets.len() == values.len() + 1"] # [doc = " should hold."] targets : SmallVec < [BasicBlockId < 'db > ; 2] > , }
};
}
