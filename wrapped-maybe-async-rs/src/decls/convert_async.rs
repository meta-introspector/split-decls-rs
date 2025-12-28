macro_rules! deps {
    () => {
        AsyncTraitMode!();
    };
}

macro_rules! convert_async {
    () => {
        deps!();
        fn convert_async (input : & mut Item , async_trait_mode : AsyncTraitMode) -> TokenStream2 { match input { Item :: Trait (item) => match async_trait_mode { AsyncTraitMode :: Send => quote ! (# [async_trait :: async_trait] # item) , AsyncTraitMode :: NotSend => quote ! (# [async_trait :: async_trait (? Send)] # item) , AsyncTraitMode :: Off => quote ! (# item) , } , Item :: Impl (item) => { let async_trait_mode = item . trait_ . as_ref () . map_or (AsyncTraitMode :: Off , | _ | async_trait_mode) ; match async_trait_mode { AsyncTraitMode :: Send => quote ! (# [async_trait :: async_trait] # item) , AsyncTraitMode :: NotSend => quote ! (# [async_trait :: async_trait (? Send)] # item) , AsyncTraitMode :: Off => quote ! (# item) , } } Item :: Fn (item) => quote ! (# item) , Item :: Static (item) => quote ! (# item) , } }
    };
}

convert_async!()