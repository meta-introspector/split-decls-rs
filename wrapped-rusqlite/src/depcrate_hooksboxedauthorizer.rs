// Generated macro for BoxedAuthorizer (type)
macro_rules! Depcrate_hooksBoxedAuthorizer {
() => {
// Module: crate::hooks
// Provides: {"BoxedAuthorizer"}
// Dependencies: {}
pub (crate) type BoxedAuthorizer = Box < dyn for < 'c > FnMut (AuthContext < 'c >) -> Authorization + Send + 'static > ;
};
}
