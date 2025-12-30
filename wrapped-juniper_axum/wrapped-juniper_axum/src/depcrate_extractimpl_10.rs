// Generated macro for impl_10 (impl)
macro_rules! Depcrate_extractimpl_10 {
() => {
// Module: crate::extract
// Provides: {"impl_10"}
// Dependencies: {}
impl < S : ScalarValue > TryFrom < GetRequest > for GraphQLRequest < S > { type Error = serde_json :: Error ; fn try_from (req : GetRequest) -> Result < Self , Self :: Error > { let GetRequest { query , operation_name , variables , } = req ; Ok (Self :: new (query , operation_name , variables . map (| v | serde_json :: from_str (& v)) . transpose () ? ,)) } }
};
}
