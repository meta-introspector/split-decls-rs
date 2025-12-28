macro_rules! ResponseUrl {
    () => {
        # [derive (Debug , Clone , PartialEq)] pub (crate) struct ResponseUrl (pub Url) ;
    };
}

ResponseUrl!()