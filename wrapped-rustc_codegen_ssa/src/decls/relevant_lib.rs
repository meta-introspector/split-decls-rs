macro_rules! deps {
    () => {
        NativeLib!();
    };
}

macro_rules! relevant_lib {
    () => {
        deps!();
        fn relevant_lib (sess : & Session , lib : & NativeLib) -> bool { match lib . cfg { Some (ref cfg) => { eval_config_entry (sess , cfg , CRATE_NODE_ID , None , ShouldEmit :: ErrorsAndLints) . as_bool () } None => true , } }
    };
}

relevant_lib!();