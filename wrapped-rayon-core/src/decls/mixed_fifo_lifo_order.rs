macro_rules! mixed_fifo_lifo_order {
    () => {
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn mixed_fifo_lifo_order () { let vec = test_mixed_order ! (spawn_fifo , spawn) ; let expected = vec ! [0 , - 3 , 1 , - 2 , 2 , - 1 , 3] ; assert_eq ! (vec , expected) ; }
    };
}

mixed_fifo_lifo_order!();