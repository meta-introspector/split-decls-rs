// Generated macro for impl_20 (impl)
macro_rules! Depcrate_emptyimpl_20 {
() => {
// Module: crate::empty
// Provides: {"impl_20"}
// Dependencies: {}
impl EmptyDataProvider { # [doc = " Creates a data provider that always returns [`DataErrorKind::MarkerNotFound`]."] pub fn new () -> Self { Self { error_kind : DataErrorKind :: MarkerNotFound , } } # [doc = " Creates a data provider that always returns the specified error kind."] pub fn new_with_error_kind (error_kind : DataErrorKind) -> Self { Self { error_kind } } }
};
}
