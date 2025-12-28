macro_rules! testable_result_err {
    () => {
        # [test] # [should_panic] fn testable_result_err () { quickcheck (Err :: < bool , i32 > as fn (i32) -> Result < bool , i32 >) ; }
    };
}

testable_result_err!()