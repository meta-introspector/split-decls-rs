// Generated macro for impl_75 (impl)
macro_rules! Depcrate_initializerimpl_75 {
() => {
// Module: crate::initializer
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'a > ToTokens for Initializer < 'a > { fn to_tokens (& self , tokens : & mut TokenStream) { let struct_field = & self . field_ident ; let builder_field = struct_field ; let append_rhs = | tokens : & mut TokenStream | { if ! self . field_enabled { let default = self . default () ; tokens . append_all (quote ! (# default)) ; } else { match & self . conversion { FieldConversion :: Block (conv) => { conv . to_tokens (tokens) ; } FieldConversion :: Move => tokens . append_all (quote ! (self .# builder_field)) , FieldConversion :: OptionOrDefault => { let match_some = self . match_some () ; let match_none = self . match_none () ; tokens . append_all (quote ! (match self .# builder_field { # match_some , # match_none , })) ; } } } } ; tokens . append_all (quote ! (# struct_field :)) ; append_rhs (tokens) ; tokens . append_all (quote ! (,)) ; } }
};
}
