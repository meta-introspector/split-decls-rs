macro_rules! http {
    () => {
        # [doc = ""] # [cfg (feature = "http-client")] pub mod http ;
    };
}

http!();