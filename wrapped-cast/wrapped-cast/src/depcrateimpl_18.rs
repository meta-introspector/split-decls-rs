// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl Error { # [doc = " A private helper function that implements `description`, because"] # [doc = " `description` is only available when we have `std` enabled."] fn description_helper (& self) -> & str { match * self { Error :: Infinite => "Cannot store infinite value in finite type" , Error :: NaN => "Cannot store NaN in type which does not support it" , Error :: Overflow => "Overflow during numeric conversion" , Error :: Underflow => "Underflow during numeric conversion" , } } }
};
}
