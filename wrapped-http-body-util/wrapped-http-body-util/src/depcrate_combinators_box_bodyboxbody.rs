// Generated macro for BoxBody (struct)
macro_rules! Depcrate_combinators_box_bodyBoxBody {
() => {
// Module: crate::combinators::box_body
// Provides: {"BoxBody"}
// Dependencies: {}
# [doc = " A boxed [`Body`] trait object."] pub struct BoxBody < D , E > { inner : Pin < Box < dyn Body < Data = D , Error = E > + Send + Sync + 'static > > , }
};
}
