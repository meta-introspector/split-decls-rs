// Generated macro for to_nonzero (function)
macro_rules! Depcrateto_nonzero {
() => {
// Module: crate
// Provides: {"to_nonzero"}
// Dependencies: {}
const fn to_nonzero (n : Option < u32 >) -> Option < NonZero < u32 > > { match n { None => None , Some (n) => NonZero :: new (n) , } }
};
}
