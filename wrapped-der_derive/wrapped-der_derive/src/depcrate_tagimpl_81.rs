// Generated macro for impl_81 (impl)
macro_rules! Depcrate_tagimpl_81 {
() => {
// Module: crate::tag
// Provides: {"impl_81"}
// Dependencies: {}
impl Parse for TagMode { fn parse (input : syn :: parse :: ParseStream < '_ >) -> syn :: Result < Self > { let s : LitStr = input . parse () ? ; match s . value () . as_str () { "EXPLICIT" | "explicit" => Ok (TagMode :: Explicit) , "IMPLICIT" | "implicit" => Ok (TagMode :: Implicit) , _ => Err (syn :: Error :: new (s . span () , "invalid tag mode (supported modes are `EXPLICIT` and `IMPLICIT`)" ,)) , } } }
};
}
