// Generated macro for fields_in_variant (function)
macro_rules! Depcrate_item_enumfields_in_variant {
() => {
// Module: crate::item_enum
// Provides: {"fields_in_variant"}
// Dependencies: {}
# [doc = " Return the correct syntax to pattern match on the enum variant, discarding all"] # [doc = " internal field data."] fn fields_in_variant (variant : & syn :: Variant) -> TokenStream { match & variant . fields { syn :: Fields :: Unnamed (_) => quote_spanned ! { variant . span () => (..) } , syn :: Fields :: Unit => quote_spanned ! { variant . span () => } , syn :: Fields :: Named (_) => quote_spanned ! { variant . span () => { .. } } , } }
};
}
