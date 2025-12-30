// Generated macro for impl_32 (impl)
macro_rules! Depcrateimpl_32 {
() => {
// Module: crate
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a > PasswordHashRef < 'a > { # [doc = " Parse the given input string, returning an [`PasswordHashRef`] if valid."] pub fn new (s : & 'a str) -> Result < Self > { validate (s) ? ; Ok (Self (s)) } # [doc = " Get the contained string as a `str`."] pub fn as_str (self) -> & 'a str { self . 0 } # [doc = " Get the algorithm identifier for this MCF hash."] pub fn id (self) -> & 'a str { Fields :: new (self . as_str ()) . next () . expect (INVARIANT_MSG) . as_str () } # [doc = " Get an iterator over the parts of the password hash as delimited by `$`, excluding the"] # [doc = " initial identifier."] pub fn fields (self) -> Fields < 'a > { let mut fields = Fields :: new (self . as_str ()) ; let id = fields . next () . expect (INVARIANT_MSG) ; debug_assert_eq ! (self . id () , id . as_str ()) ; fields } }
};
}
