// Generated macro for impl_27 (impl)
macro_rules! Depcrate_lib_oxideimpl_27 {
() => {
// Module: crate::lib_oxide
// Provides: {"impl_27"}
// Dependencies: {}
impl StateType for InflateState { const STATE_TYPE : StateTypeEnum = StateTypeEnum :: InflateType ; fn from_enum (value : & mut InternalState) -> Option < & mut Self > { if let InternalState :: Inflate (state) = value { Some (state . as_mut ()) } else { None } } }
};
}
