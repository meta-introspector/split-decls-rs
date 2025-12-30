// Generated macro for r#impl (macro)
macro_rules! Depcrate_cmpr#impl {
() => {
// Module: crate::cmp
// Provides: {"r#impl"}
// Dependencies: {}
macro_rules ! r#impl { ($ left : ty , $ right : ty) => { impl PartialEq <$ right > for $ left { # [inline] fn eq (& self , other : &$ right) -> bool { < BasePath as PartialEq < Path >>:: eq (self , other . as_ref ()) } } impl PartialEq <$ left > for $ right { # [inline] fn eq (& self , other : &$ left) -> bool { other == self } } impl PartialOrd <$ right > for $ left { # [inline] fn partial_cmp (& self , other : &$ right) -> Option < Ordering > { < BasePath as PartialOrd < Path >>:: partial_cmp (self , other . as_ref () ,) } } impl PartialOrd <$ left > for $ right { # [inline] fn partial_cmp (& self , other : &$ left) -> Option < Ordering > { other . partial_cmp (self) } } } ; }
};
}
