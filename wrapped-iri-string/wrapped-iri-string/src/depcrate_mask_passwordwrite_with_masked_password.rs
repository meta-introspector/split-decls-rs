// Generated macro for write_with_masked_password (function)
macro_rules! Depcrate_mask_passwordwrite_with_masked_password {
() => {
// Module: crate::mask_password
// Provides: {"write_with_masked_password"}
// Dependencies: {}
# [doc = " Writes the URI with the password part replaced."] fn write_with_masked_password < D > (f : & mut fmt :: Formatter < '_ > , s : & str , pw_range : Range < usize > , alt : & D ,) -> fmt :: Result where D : ? Sized + fmt :: Display , { debug_assert ! (s . len () >= pw_range . end , "[consistency] password range must be inside the IRI") ; f . write_str (& s [.. pw_range . start]) ? ; alt . fmt (f) ? ; f . write_str (& s [pw_range . end ..]) ? ; Ok (()) }
};
}
