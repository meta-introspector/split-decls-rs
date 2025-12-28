macro_rules! test_can_print_warnings {
    () => {
        # [test] fn test_can_print_warnings () { sess_and_cfg (& ["-Awarnings"] , | sess , _cfg | { assert ! (! sess . dcx () . can_emit_warnings ()) ; }) ; sess_and_cfg (& ["-Awarnings" , "-Dwarnings"] , | sess , _cfg | { assert ! (sess . dcx () . can_emit_warnings ()) ; }) ; sess_and_cfg (& ["-Adead_code"] , | sess , _cfg | { assert ! (sess . dcx () . can_emit_warnings ()) ; }) ; }
    };
}

test_can_print_warnings!();