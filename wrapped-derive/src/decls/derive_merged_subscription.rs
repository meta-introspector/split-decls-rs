macro_rules! deps {
    () => {
        MergedSubscription!();
    };
}

macro_rules! derive_merged_subscription {
    () => {
        deps!();
        # [proc_macro_derive (MergedSubscription , attributes (graphql))] pub fn derive_merged_subscription (input : TokenStream) -> TokenStream { let object_args = match args :: MergedSubscription :: from_derive_input (& parse_macro_input ! (input as DeriveInput)) { Ok (object_args) => object_args , Err (err) => return TokenStream :: from (err . write_errors ()) , } ; match merged_subscription :: generate (& object_args) { Ok (expanded) => expanded , Err (err) => err . write_errors () . into () , } }
    };
}

derive_merged_subscription!();