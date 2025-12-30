// Generated macro for impl_305 (impl)
macro_rules! Depcrate_exactly_one_errimpl_305 {
() => {
// Module: crate::exactly_one_err
// Provides: {"impl_305"}
// Dependencies: {}
impl < I > ExactlyOneError < I > where I : Iterator , { # [doc = " Creates a new `ExactlyOneErr` iterator."] pub (crate) fn new (first_two : Option < Either < [I :: Item ; 2] , I :: Item > > , inner : I) -> Self { Self { first_two , inner } } fn additional_len (& self) -> usize { match self . first_two { Some (Either :: Left (_)) => 2 , Some (Either :: Right (_)) => 1 , None => 0 , } } }
};
}
