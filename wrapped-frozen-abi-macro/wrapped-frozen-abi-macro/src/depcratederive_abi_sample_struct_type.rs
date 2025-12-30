// Generated macro for derive_abi_sample_struct_type (function)
macro_rules! Depcratederive_abi_sample_struct_type {
() => {
// Module: crate
// Provides: {"derive_abi_sample_struct_type"}
// Dependencies: {}
# [cfg (feature = "frozen-abi")] fn derive_abi_sample_struct_type (input : ItemStruct) -> TokenStream { let type_name = & input . ident ; let fields = & input . fields ; let mut sample_fields = quote ! { } ; match fields { Fields :: Named (_) => { for field in fields { let field_name = & field . ident ; sample_fields . extend (quote ! { # field_name : AbiExample :: example () , }) ; } sample_fields = quote ! { { # sample_fields } } ; } Fields :: Unnamed (_) => { for _ in fields { sample_fields . extend (quote ! { AbiExample :: example () , }) ; } sample_fields = quote ! { (# sample_fields) } ; } _ => unimplemented ! ("fields: {:?}" , fields) , } let mut attrs = input . attrs . clone () ; filter_allow_attrs (& mut attrs) ; let (impl_generics , ty_generics , where_clause) = input . generics . split_for_impl () ; let turbofish = ty_generics . as_turbofish () ; let result = quote ! { # [automatically_derived] # (# attrs) * impl # impl_generics :: solana_frozen_abi :: abi_example :: AbiExample for # type_name # ty_generics # where_clause { fn example () -> Self { :: std :: println ! ("AbiExample::example for struct: {}" , std :: any :: type_name ::<# type_name # ty_generics > ()) ; use :: solana_frozen_abi :: abi_example :: AbiExample ; # type_name # turbofish # sample_fields } } } ; result . into () }
};
}
