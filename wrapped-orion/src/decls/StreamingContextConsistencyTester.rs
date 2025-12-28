macro_rules! StreamingContextConsistencyTester {
    () => {
        # [allow (dead_code)] # [doc = " A streaming context tester."] pub struct StreamingContextConsistencyTester < R , T > { _return_type : PhantomData < R > , _initial_context : T , blocksize : usize , }
    };
}

StreamingContextConsistencyTester!()