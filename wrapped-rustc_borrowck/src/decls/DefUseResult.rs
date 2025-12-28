macro_rules! DefUseResult {
    () => {
        enum DefUseResult { Def , UseLive { local : Local } , UseDrop { local : Local } , }
    };
}

DefUseResult!()