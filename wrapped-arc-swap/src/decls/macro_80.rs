macro_rules! deps {
    () => {
        LocalNode!();
    };
}

macro_rules! macro_80 {
    () => {
        deps!();
        # [cfg (not (feature = "experimental-thread-local"))] thread_local ! { # [doc = " A debt node assigned to this thread."] static THREAD_HEAD : LocalNode = LocalNode { node : Cell :: new (None) , fast : FastLocal :: default () , helping : HelpingLocal :: default () , } ; }
    };
}

macro_80!()