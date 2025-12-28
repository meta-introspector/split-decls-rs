macro_rules! get_unchecked_mut {
    () => {
        # [inline] unsafe fn get_unchecked_mut < K > (xs : & mut [K] , index : usize) -> & mut K { unsafe { debug_assert ! (index < xs . len ()) ; xs . get_unchecked_mut (index) } }
    };
}

get_unchecked_mut!();