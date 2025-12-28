macro_rules! testable_result {
    () => {
        # [test] fn testable_result () { fn result () -> Result < bool , String > { Ok (true) } quickcheck (result as fn () -> Result < bool , String >) ; }
    };
}

testable_result!()