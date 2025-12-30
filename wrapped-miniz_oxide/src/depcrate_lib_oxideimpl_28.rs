// Generated macro for impl_28 (impl)
macro_rules! Depcrate_lib_oxideimpl_28 {
() => {
// Module: crate::lib_oxide
// Provides: {"impl_28"}
// Dependencies: {}
impl StateType for Compressor { const STATE_TYPE : StateTypeEnum = StateTypeEnum :: DeflateType ; fn from_enum (value : & mut InternalState) -> Option < & mut Self > { if let InternalState :: Deflate (state) = value { Some (state . as_mut ()) } else { None } } }
};
}
