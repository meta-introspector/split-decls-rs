// Generated macro for impl_uprim_vanilla (macro)
macro_rules! Depcrate_reducedimpl_uprim_vanilla {
() => {
// Module: crate::reduced
// Provides: {"impl_uprim_vanilla"}
// Dependencies: {}
macro_rules ! impl_uprim_vanilla { ($ t : ident , $ ns : ident) => { mod $ ns { use super ::*; use crate :: word ::$ t ::*; impl Reducer <$ t > for Vanilla <$ t > { impl_uprim_vanilla_core ! ($ t) ; # [inline] fn mul (& self , lhs : &$ t , rhs : &$ t) -> $ t { (wmul (* lhs , * rhs) % extend (self . 0)) as $ t } # [inline] fn sqr (& self , target : $ t) -> $ t { (wsqr (target) % extend (self . 0)) as $ t } } } } ; }
};
}
