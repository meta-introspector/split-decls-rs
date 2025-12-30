// Generated macro for should_lint (function)
macro_rules! Depcrate_casts_cast_sign_lossshould_lint {
() => {
// Module: crate::casts::cast_sign_loss
// Provides: {"should_lint"}
// Dependencies: {}
fn should_lint < 'cx > (cx : & LateContext < 'cx > , cast_op : & Expr < '_ > , cast_from : Ty < 'cx > , cast_to : Ty < '_ >) -> bool { match (cast_from . is_integral () , cast_to . is_integral ()) { (true , true) => { if ! cast_from . is_signed () || cast_to . is_signed () { return false ; } if let Sign :: ZeroOrPositive = expr_sign (cx , cast_op , cast_from) { return false ; } if let Sign :: ZeroOrPositive = expr_muldiv_sign (cx , cast_op) { return false ; } if let Sign :: ZeroOrPositive = expr_add_sign (cx , cast_op) { return false ; } true } , (false , true) => ! cast_to . is_signed () , (_ , _) => false , } }
};
}
