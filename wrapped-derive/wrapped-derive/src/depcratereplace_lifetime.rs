// Generated macro for replace_lifetime (function)
macro_rules! Depcratereplace_lifetime {
() => {
// Module: crate
// Provides: {"replace_lifetime"}
// Dependencies: {}
fn replace_lifetime (x : & Type , lt : Lifetime) -> Type { use syn :: fold :: Fold ; struct ReplaceLifetime (Lifetime) ; impl Fold for ReplaceLifetime { fn fold_lifetime (& mut self , _ : Lifetime) -> Lifetime { self . 0 . clone () } } ReplaceLifetime (lt) . fold_type (x . clone ()) }
};
}
