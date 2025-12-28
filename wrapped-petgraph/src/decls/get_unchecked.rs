macro_rules! get_unchecked {
    () => {
        # [inline] unsafe fn get_unchecked < K > (xs : & [K] , index : usize) -> & K { unsafe { debug_assert ! (index < xs . len ()) ; xs . get_unchecked (index) } }
    };
}

get_unchecked!()