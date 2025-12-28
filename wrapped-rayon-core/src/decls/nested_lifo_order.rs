macro_rules! nested_lifo_order {
    () => {
        # [test] # [cfg_attr (any (target_os = "emscripten" , target_family = "wasm") , ignore)] fn nested_lifo_order () { let vec = test_nested_order ! (scope => spawn , scope => spawn) ; let expected : Vec < i32 > = (0 .. 100) . rev () . collect () ; assert_eq ! (vec , expected) ; }
    };
}

nested_lifo_order!();