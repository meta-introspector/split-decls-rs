macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! arg_send_sync {
    () => {
        deps!();
        # [test] fn arg_send_sync () { fn foo < T : Send + Sync > (_ : T) { } foo (Arg :: new ("test")) ; }
    };
}

arg_send_sync!()