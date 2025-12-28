macro_rules! deps {
    () => {
        MergedObject!();
    };
}

macro_rules! derive_merged_object {
    () => {
        deps!();
        # [proc_macro_derive (MergedObject , attributes (graphql))] pub fn derive_merged_object (input : TokenStream) -> TokenStream { let object_args = match args :: MergedObject :: from_derive_input (& parse_macro_input ! (input as DeriveInput)) { Ok (object_args) => object_args , Err (err) => return TokenStream :: from (err . write_errors ()) , } ; match merged_object :: generate (& object_args) { Ok (expanded) => expanded , Err (err) => err . write_errors () . into () , } }
    };
}

derive_merged_object!();