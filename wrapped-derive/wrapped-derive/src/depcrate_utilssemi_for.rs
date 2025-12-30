// Generated macro for semi_for (function)
macro_rules! Depcrate_utilssemi_for {
() => {
// Module: crate::utils
// Provides: {"semi_for"}
// Dependencies: {}
# [doc = " Return a semicolon token if necessary after the struct definition"] pub fn semi_for (f : & Fields) -> TokenStream2 { if let Fields :: Unnamed (..) = * f { quote ! (;) } else { quote ! () } }
};
}
