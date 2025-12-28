macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! shrink_to_fit_unspill {
    () => {
        deps!();
        # [test] fn shrink_to_fit_unspill () { let mut vec = SmallVec :: < u8 , 2 > :: from_iter (0 .. 3) ; vec . pop () ; assert ! (vec . spilled ()) ; vec . shrink_to_fit () ; assert ! (! vec . spilled () , "shrink_to_fit will un-spill if possible") ; }
    };
}

shrink_to_fit_unspill!();