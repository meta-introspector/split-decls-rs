// Generated macro for fresh_lifetime (function)
macro_rules! Depcratefresh_lifetime {
() => {
// Module: crate
// Provides: {"fresh_lifetime"}
// Dependencies: {}
# [doc = " Generate a new lifetime that doesn't conflict with the existing lifetimes."] fn fresh_lifetime < 'a > (lifetimes : impl Iterator < Item = & 'a LifetimeDef > , name : & str) -> Lifetime { let mut used_idents = HashSet :: new () ; for lifetime in lifetimes { used_idents . insert (lifetime . lifetime . ident . to_string ()) ; } let mut lifetime_name = name . to_owned () ; let mut counter = 0 ; while used_idents . contains (& lifetime_name) { use std :: fmt :: Write ; counter += 1 ; lifetime_name . clear () ; write ! (& mut lifetime_name , "{}{}" , name , counter) . unwrap () ; } Lifetime :: new (& format ! ("'{}" , lifetime_name) , Span :: call_site ()) }
};
}
