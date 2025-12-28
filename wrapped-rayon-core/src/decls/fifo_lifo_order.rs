macro_rules! fifo_lifo_order {
    () => {
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn fifo_lifo_order () { let vec = test_order ! (spawn_fifo , spawn) ; let expected : Vec < i32 > = (0 .. 10) . flat_map (| i | (0 .. 10) . rev () . map (move | j | i * 10 + j)) . collect () ; assert_eq ! (vec , expected) ; }
    };
}

fifo_lifo_order!()