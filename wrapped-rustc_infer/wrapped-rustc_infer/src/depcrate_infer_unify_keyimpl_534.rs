// Generated macro for impl_534 (impl)
macro_rules! Depcrate_infer_unify_keyimpl_534 {
() => {
// Module: crate::infer::unify_key
// Provides: {"impl_534"}
// Dependencies: {}
impl < 'tcx > UnifyValue for RegionVariableValue < 'tcx > { type Error = RegionUnificationError ; fn unify_values (value1 : & Self , value2 : & Self) -> Result < Self , Self :: Error > { match (* value1 , * value2) { (RegionVariableValue :: Known { .. } , RegionVariableValue :: Known { .. }) => { Err (RegionUnificationError) } (RegionVariableValue :: Known { value } , RegionVariableValue :: Unknown { universe }) | (RegionVariableValue :: Unknown { universe } , RegionVariableValue :: Known { value }) => { let universe_of_value = match value . kind () { ty :: ReStatic | ty :: ReErased | ty :: ReLateParam (..) | ty :: ReEarlyParam (..) | ty :: ReError (_) => ty :: UniverseIndex :: ROOT , ty :: RePlaceholder (placeholder) => placeholder . universe , ty :: ReVar (..) | ty :: ReBound (..) => bug ! ("not a universal region") , } ; if universe . can_name (universe_of_value) { Ok (RegionVariableValue :: Known { value }) } else { Err (RegionUnificationError) } } (RegionVariableValue :: Unknown { universe : a } , RegionVariableValue :: Unknown { universe : b } ,) => { Ok (RegionVariableValue :: Unknown { universe : a . min (b) }) } } } }
};
}
