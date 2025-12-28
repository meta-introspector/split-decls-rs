macro_rules! raw_deprecated {
    () => {
        # [cfg (not (feature = "raw-deprecated"))] pub (crate) fn raw_deprecated () -> TokenStream { quote ! { #! [allow (deprecated)] } }
    };
}

raw_deprecated!()