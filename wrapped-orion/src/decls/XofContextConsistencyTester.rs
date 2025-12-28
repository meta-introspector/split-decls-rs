macro_rules! XofContextConsistencyTester {
    () => {
        # [allow (dead_code)] # [doc = " A streaming context tester."] pub struct XofContextConsistencyTester < T > { _initial_context : T , blocksize : usize , }
    };
}

XofContextConsistencyTester!()