// Generated macro for Any (struct)
macro_rules! Depcrate_anyAny {
() => {
// Module: crate::any
// Provides: {"Any"}
// Dependencies: {}
pub struct Any { value : Value , drop : unsafe fn (& mut Value) , type_id : TypeId , # [doc = " For panic messages only. Not used for comparison."] # [cfg (feature = "unstable-debug")] type_name : & 'static str , }
};
}
