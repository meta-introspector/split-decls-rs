// Generated macro for update_nullability (function)
macro_rules! Depcrate_rust_typeupdate_nullability {
() => {
// Module: crate::rust_type
// Provides: {"update_nullability"}
// Dependencies: {}
fn update_nullability (nullability : & mut Nullability , new : Option < Nullability >) { match (* nullability , new) { (_ , None) => { } (Nullability :: Unspecified , Some (new)) => { * nullability = new ; } (old , new) => error ! (? old , ? new , "invalid nullability update") , } }
};
}
