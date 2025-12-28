macro_rules! curl {
    () => {
        # [cfg (feature = "http-client-curl")] # [doc = ""] pub mod curl ;
    };
}

curl!()