macro_rules! GraphiQLVersion {
    () => {
        # [derive (Serialize)] struct GraphiQLVersion < 'a > (& 'a str) ;
    };
}

GraphiQLVersion!()