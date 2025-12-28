macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! app_send_sync {
    () => {
        deps!();
        # [test] fn app_send_sync () { fn foo < T : Send + Sync > (_ : T) { } foo (Command :: new ("test")) ; }
    };
}

app_send_sync!();