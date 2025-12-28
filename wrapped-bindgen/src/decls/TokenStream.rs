macro_rules! TokenStream {
    () => {
        # [doc = " A stream of tokens"] # [derive (Debug , Clone , PartialEq , Eq , PartialOrd , Ord)] pub struct TokenStream (pub String) ;
    };
}

TokenStream!()