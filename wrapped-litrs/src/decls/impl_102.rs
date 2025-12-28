macro_rules! deps {
    () => {
        Span!();
        Literal!();
        InvalidToken!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl InvalidToken { # [doc = " Returns a token stream representing `compile_error!(\"msg\");` where"] # [doc = " `\"msg\"` is the output of `self.to_string()`. **Panics if called outside"] # [doc = " of a proc-macro context!**"] pub fn to_compile_error (& self) -> proc_macro :: TokenStream { use proc_macro :: { Delimiter , Group , Ident , Punct , Spacing , TokenTree } ; let span = match self . span { Span :: One (s) => s , # [cfg (feature = "proc-macro2")] Span :: Two (s) => s . unwrap () , } ; let msg = self . to_string () ; let tokens = vec ! [TokenTree :: from (Ident :: new ("compile_error" , span)) , TokenTree :: from (Punct :: new ('!' , Spacing :: Alone)) , TokenTree :: from (Group :: new (Delimiter :: Parenthesis , TokenTree :: from (proc_macro :: Literal :: string (& msg)) . into () ,)) ,] ; tokens . into_iter () . map (| mut t | { t . set_span (span) ; t }) . collect () } # [doc = " Like [`to_compile_error`][Self::to_compile_error], but returns a token"] # [doc = " stream from `proc_macro2` and does not panic outside of a proc-macro"] # [doc = " context."] # [cfg (feature = "proc-macro2")] pub fn to_compile_error2 (& self) -> proc_macro2 :: TokenStream { use proc_macro2 :: { Delimiter , Group , Ident , Punct , Spacing , TokenTree } ; let span = match self . span { Span :: One (s) => proc_macro2 :: Span :: from (s) , Span :: Two (s) => s , } ; let msg = self . to_string () ; let tokens = vec ! [TokenTree :: from (Ident :: new ("compile_error" , span)) , TokenTree :: from (Punct :: new ('!' , Spacing :: Alone)) , TokenTree :: from (Group :: new (Delimiter :: Parenthesis , TokenTree :: from (proc_macro2 :: Literal :: string (& msg)) . into () ,)) ,] ; tokens . into_iter () . map (| mut t | { t . set_span (span) ; t }) . collect () } }
    };
}

impl_102!()