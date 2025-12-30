// Generated macro for insert_lifetime (function)
macro_rules! Depcrate_utilsinsert_lifetime {
() => {
// Module: crate::utils
// Provides: {"insert_lifetime"}
// Dependencies: {}
# [doc = " Inserts a `lifetime` at position `0` of `generics.params`."] pub (crate) fn insert_lifetime (generics : & mut Generics , lifetime : Lifetime) { generics . lt_token . get_or_insert_with (< Token ! [<] > :: default) ; generics . gt_token . get_or_insert_with (< Token ! [>] > :: default) ; generics . params . insert (0 , LifetimeParam :: new (lifetime) . into ()) ; }
};
}
