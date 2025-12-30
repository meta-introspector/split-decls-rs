// Generated macro for impl_8 (impl)
macro_rules! Depcrate_extimpl_8 {
() => {
// Module: crate::ext
// Provides: {"impl_8"}
// Dependencies: {}
impl TokenStreamExt for TokenStream { fn append < U > (& mut self , token : U) where U : Into < TokenTree > , { self . extend (iter :: once (token . into ())) ; } fn append_all < I > (& mut self , iter : I) where I : IntoIterator , I :: Item : ToTokens , { do_append_all (self , iter . into_iter ()) ; fn do_append_all < I > (stream : & mut TokenStream , iter : I) where I : Iterator , I :: Item : ToTokens , { for token in iter { token . to_tokens (stream) ; } } } fn append_separated < I , U > (& mut self , iter : I , op : U) where I : IntoIterator , I :: Item : ToTokens , U : ToTokens , { do_append_separated (self , iter . into_iter () , op) ; fn do_append_separated < I , U > (stream : & mut TokenStream , iter : I , op : U) where I : Iterator , I :: Item : ToTokens , U : ToTokens , { for (i , token) in iter . into_iter () . enumerate () { if i > 0 { op . to_tokens (stream) ; } token . to_tokens (stream) ; } } } fn append_terminated < I , U > (& mut self , iter : I , term : U) where I : IntoIterator , I :: Item : ToTokens , U : ToTokens , { do_append_terminated (self , iter . into_iter () , term) ; fn do_append_terminated < I , U > (stream : & mut TokenStream , iter : I , term : U) where I : Iterator , I :: Item : ToTokens , U : ToTokens , { for token in iter { token . to_tokens (stream) ; term . to_tokens (stream) ; } } } }
};
}
