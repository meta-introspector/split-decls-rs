macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! test_drop_panic_smallvec {
    () => {
        deps!();
        # [test] # [should_panic] fn test_drop_panic_smallvec () { struct DropPanic ; impl Drop for DropPanic { fn drop (& mut self) { panic ! ("drop") ; } } let mut v = SmallVec :: < _ , 1 > :: new () ; v . push (DropPanic) ; }
    };
}

test_drop_panic_smallvec!()