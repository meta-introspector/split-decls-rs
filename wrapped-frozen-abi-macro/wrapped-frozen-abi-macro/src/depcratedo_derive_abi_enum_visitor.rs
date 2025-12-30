// Generated macro for do_derive_abi_enum_visitor (function)
macro_rules! Depcratedo_derive_abi_enum_visitor {
() => {
// Module: crate
// Provides: {"do_derive_abi_enum_visitor"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] fn do_derive_abi_enum_visitor (input : ItemEnum) -> TokenStream { let type_name = & input . ident ; let mut serialized_variants = quote ! { } ; let mut variant_count : u64 = 0 ; let (impl_generics , ty_generics , where_clause) = input . generics . split_for_impl () ; for variant in & input . variants { if filter_serde_attrs (& variant . attrs) { continue ; } ; let sample_variant = quote_sample_variant (type_name , & ty_generics , variant) ; variant_count = if let Some (variant_count) = variant_count . checked_add (1) { variant_count } else { break ; } ; serialized_variants . extend (quote ! { # sample_variant ; Serialize :: serialize (& sample_variant , digester . create_enum_child () ?) ?; }) ; } let type_str = format ! ("{type_name}") ; (quote ! { impl # impl_generics :: solana_frozen_abi :: abi_example :: AbiEnumVisitor for # type_name # ty_generics # where_clause { fn visit_for_abi (& self , digester : & mut :: solana_frozen_abi :: abi_digester :: AbiDigester) -> :: solana_frozen_abi :: abi_digester :: DigestResult { let enum_name = # type_str ; use :: serde :: ser :: Serialize ; use :: solana_frozen_abi :: abi_example :: AbiExample ; digester . update_with_string (:: std :: format ! ("enum {} (variants = {})" , enum_name , # variant_count)) ; # serialized_variants digester . create_child () } } }) . into () }
};
}
