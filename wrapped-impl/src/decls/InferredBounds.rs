macro_rules! InferredBounds {
    () => {
        pub struct InferredBounds { bounds : Map < String , (Set < String > , Punctuated < TokenStream , Token ! [+] >) > , order : Vec < TokenStream > , }
    };
}

InferredBounds!()