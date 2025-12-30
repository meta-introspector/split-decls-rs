// Generated macro for impl_1557 (impl)
macro_rules! Depcrateimpl_1557 {
() => {
// Module: crate
// Provides: {"impl_1557"}
// Dependencies: {}
impl std :: error :: Error for GraphQLError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Self :: ParseError (e) => Some (e) , Self :: ValidationError (errs) => Some (errs . first () ?) , Self :: NoOperationProvided | Self :: MultipleOperationsProvided | Self :: UnknownOperationName | Self :: IsSubscription | Self :: NotSubscription => None , } } }
};
}
