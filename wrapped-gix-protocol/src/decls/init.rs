macro_rules! init {
    () => {
        # [doc = ""] # [cfg (any (feature = "blocking-client" , feature = "async-client"))] pub mod init ;
    };
}

init!()