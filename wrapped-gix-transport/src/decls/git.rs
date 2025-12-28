macro_rules! git {
    () => {
        # [doc = ""] # [cfg (any (feature = "blocking-client" , feature = "async-client"))] pub mod git ;
    };
}

git!();