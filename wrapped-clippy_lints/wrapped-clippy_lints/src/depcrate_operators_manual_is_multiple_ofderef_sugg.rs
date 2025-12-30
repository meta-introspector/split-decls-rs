// Generated macro for deref_sugg (function)
macro_rules! Depcrate_operators_manual_is_multiple_ofderef_sugg {
() => {
// Module: crate::operators::manual_is_multiple_of
// Provides: {"deref_sugg"}
// Dependencies: {}
fn deref_sugg < 'a > (sugg : Sugg < 'a > , ty : Ty < '_ >) -> Sugg < 'a > { if let ty :: Ref (_ , target_ty , _) = ty . kind () { deref_sugg (sugg . deref () , * target_ty) } else { sugg } }
};
}
