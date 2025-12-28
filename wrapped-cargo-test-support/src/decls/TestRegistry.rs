macro_rules! deps {
    () => {
        Package!();
        HttpServerHandle!();
        Token!();
    };
}

macro_rules! TestRegistry {
    () => {
        deps!();
        # [doc = " A local registry fixture"] # [doc = ""] # [doc = " Most tests won't need to call this directly but instead interact with [`Package`]"] pub struct TestRegistry { server : Option < HttpServerHandle > , index_url : Url , path : PathBuf , api_url : Url , dl_url : Url , token : Token , }
    };
}

TestRegistry!();