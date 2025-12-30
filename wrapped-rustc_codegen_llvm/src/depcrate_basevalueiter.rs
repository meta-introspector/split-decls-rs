// Generated macro for ValueIter (struct)
macro_rules! Depcrate_baseValueIter {
() => {
// Module: crate::base
// Provides: {"ValueIter"}
// Dependencies: {}
pub (crate) struct ValueIter < 'll > { cur : Option < & 'll Value > , step : unsafe extern "C" fn (& 'll Value) -> Option < & 'll Value > , }
};
}
