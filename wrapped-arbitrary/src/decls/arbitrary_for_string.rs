macro_rules! arbitrary_for_string {
    () => {
        # [test] fn arbitrary_for_string () { assert_generates :: < String > (["" . into () , "a" . into () , "aa" . into () , "aaa" . into ()]) ; }
    };
}

arbitrary_for_string!();