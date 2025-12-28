macro_rules! yield_now_to_spawn {
    () => {
        # [test] fn yield_now_to_spawn () { let (tx , rx) = channel () ; crate :: spawn (move | | tx . send (22) . unwrap ()) ; crate :: registry :: in_worker (move | _ , _ | { crate :: yield_now () ; }) ; assert_eq ! (22 , rx . recv () . unwrap ()) ; }
    };
}

yield_now_to_spawn!();