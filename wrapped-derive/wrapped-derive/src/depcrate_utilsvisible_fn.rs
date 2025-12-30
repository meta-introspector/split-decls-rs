// Generated macro for visible_fn (function)
macro_rules! Depcrate_utilsvisible_fn {
() => {
// Module: crate::utils
// Provides: {"visible_fn"}
// Dependencies: {}
pub fn visible_fn (visible : & Option < Visible >) -> TokenStream { match visible { None | Some (Visible :: None) => quote ! { :: std :: option :: Option :: None } , Some (Visible :: HiddenAlways) => quote ! { :: std :: option :: Option :: Some (| _ | false) } , Some (Visible :: FnName (name)) => { quote ! { :: std :: option :: Option :: Some (# name) } } } }
};
}
