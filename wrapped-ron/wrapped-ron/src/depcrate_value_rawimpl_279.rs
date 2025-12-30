// Generated macro for impl_279 (impl)
macro_rules! Depcrate_value_rawimpl_279 {
() => {
// Module: crate::value::raw
// Provides: {"impl_279"}
// Dependencies: {}
impl RawValue { # [must_use] # [doc = " Trims any leadning and trailing whitespace off the raw RON string,"] # [doc = " including whitespace characters and comments."] pub fn trim (& self) -> & Self { Self :: from_borrowed_str (& self . ron [RawValue :: trim_range (& self . ron)]) } # [must_use] # [allow (unsafe_code)] # [doc = " Trims any leadning and trailing whitespace off the boxed raw RON string,"] # [doc = " including whitespace characters and comments."] pub fn trim_boxed (self : Box < Self >) -> Box < Self > { let trim_range = RawValue :: trim_range (& self . ron) ; let mut boxed_ron = RawValue :: into_boxed_str (self) . into_string () ; unsafe { boxed_ron . as_mut_vec () . drain (trim_range . end ..) ; boxed_ron . as_mut_vec () . drain (0 .. trim_range . start) ; } RawValue :: from_boxed_str (boxed_ron . into_boxed_str ()) } }
};
}
