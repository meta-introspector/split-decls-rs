// Generated macro for UnsyncBoxBody (struct)
macro_rules! Depcrate_combinators_box_bodyUnsyncBoxBody {
() => {
// Module: crate::combinators::box_body
// Provides: {"UnsyncBoxBody"}
// Dependencies: {}
# [doc = " A boxed [`Body`] trait object that is !Sync."] pub struct UnsyncBoxBody < D , E > { inner : Pin < Box < dyn Body < Data = D , Error = E > + Send + 'static > > , }
};
}
