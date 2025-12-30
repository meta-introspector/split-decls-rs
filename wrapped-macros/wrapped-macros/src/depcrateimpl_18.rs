// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl TokenStreamExt for TokenStream { fn push < T > (& mut self , token : T) where T : Into < TokenTree > , { self . extend (iter :: once (token . into ())) ; } }
};
}
