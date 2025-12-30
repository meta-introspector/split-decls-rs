// Generated macro for impl_71 (impl)
macro_rules! Depcrate_combinators_map_errimpl_71 {
() => {
// Module: crate::combinators::map_err
// Provides: {"impl_71"}
// Dependencies: {}
impl < B , F > MapErr < B , F > { # [inline] pub (crate) fn new (body : B , f : F) -> Self { Self { inner : body , f } } # [doc = " Get a reference to the inner body"] pub fn get_ref (& self) -> & B { & self . inner } # [doc = " Get a mutable reference to the inner body"] pub fn get_mut (& mut self) -> & mut B { & mut self . inner } # [doc = " Get a pinned mutable reference to the inner body"] pub fn get_pin_mut (self : Pin < & mut Self >) -> Pin < & mut B > { self . project () . inner } # [doc = " Consume `self`, returning the inner body"] pub fn into_inner (self) -> B { self . inner } }
};
}
