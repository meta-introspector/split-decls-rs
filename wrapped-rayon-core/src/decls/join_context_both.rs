macro_rules! join_context_both {
    () => {
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn join_context_both () { let (a_migrated , b_migrated) = join_context (| a | a . migrated () , | b | b . migrated ()) ; assert ! (a_migrated) ; assert ! (b_migrated) ; }
    };
}

join_context_both!()