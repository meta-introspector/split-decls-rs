// Generated macro for impl_1241 (impl)
macro_rules! Depcrate_integrations_serdeimpl_1241 {
() => {
// Module: crate::integrations::serde
// Provides: {"impl_1241"}
// Dependencies: {}
impl Serialize for GraphQLError { fn serialize < S : Serializer > (& self , ser : S) -> Result < S :: Ok , S :: Error > { # [derive (Serialize)] struct Helper { message : & 'static str , } match self { Self :: ParseError (e) => [e] . serialize (ser) , Self :: ValidationError (es) => es . serialize (ser) , Self :: NoOperationProvided => [Helper { message : "Must provide an operation" , }] . serialize (ser) , Self :: MultipleOperationsProvided => [Helper { message : "Must provide operation name \
                          if query contains multiple operations" , }] . serialize (ser) , Self :: UnknownOperationName => [Helper { message : "Unknown operation" , }] . serialize (ser) , Self :: IsSubscription => [Helper { message : "Expected query, got subscription" , }] . serialize (ser) , Self :: NotSubscription => [Helper { message : "Expected subscription, got query" , }] . serialize (ser) , } } }
};
}
