// Generated macro for StringLike (trait)
macro_rules! DepcrateStringLike {
() => {
// Module: crate
// Provides: {"StringLike"}
// Dependencies: {}
# [doc = " A D-Bus string-like type - a basic (non-container) type with variable length."] # [doc = ""] # [doc = " It wraps a str, which means that it is unsized."] pub trait StringLike : ToOwned { # [doc = " The name of the type"] const NAME : & 'static str ; # [doc = " Creates a new borrowed string"] fn new (s : & str) -> Result < & Self , InvalidStringError > { Self :: is_valid (s) ? ; Ok (Self :: new_unchecked (s)) } # [doc = " Creates a new owned string"] fn new_owned < S : Into < String > > (s : S) -> Result < < Self as ToOwned > :: Owned , InvalidStringError > { let s = s . into () ; Self :: is_valid (& s) ? ; Ok (Self :: new_unchecked_owned (s)) } # [doc = " Creates a new borrowed string without actually checking that it is valid."] # [doc = ""] # [doc = " Sending this over D-Bus if actually invalid, could result in e g immediate disconnection"] # [doc = " from the server."] fn new_unchecked (_ : & str) -> & Self ; # [doc = " Creates a new owned string without actually checking that it is valid."] # [doc = ""] # [doc = " Sending this over D-Bus if actually invalid, could result in e g immediate disconnection"] # [doc = " from the server."] fn new_unchecked_owned (_ : String) -> < Self as ToOwned > :: Owned ; # [doc = " Checks whether or not a string is valid."] fn is_valid (_ : & str) -> Result < () , InvalidStringError > ; }
};
}
