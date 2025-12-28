macro_rules! scope_empty {
    () => {
        # [test] fn scope_empty () { scope (| _ | { }) ; }
    };
}

scope_empty!()