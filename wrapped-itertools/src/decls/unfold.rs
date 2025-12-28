macro_rules! deps {
    () => {
        Unfold!();
    };
}

macro_rules! unfold {
    () => {
        deps!();
        # [doc = " Creates a new unfold source with the specified closure as the \"iterator"] # [doc = " function\" and an initial state to eventually pass to the closure"] # [doc = ""] # [doc = " `unfold` is a general iterator builder: it has a mutable state value,"] # [doc = " and a closure with access to the state that produces the next value."] # [doc = ""] # [doc = " This more or less equivalent to a regular struct with an [`Iterator`]"] # [doc = " implementation, and is useful for one-off iterators."] # [doc = ""] # [doc = " ```"] # [doc = " // an iterator that yields sequential Fibonacci numbers,"] # [doc = " // and stops at the maximum representable value."] # [doc = ""] # [doc = " use itertools::unfold;"] # [doc = ""] # [doc = " let mut fibonacci = unfold((1u32, 1u32), |(x1, x2)| {"] # [doc = "     // Attempt to get the next Fibonacci number"] # [doc = "     let next = x1.saturating_add(*x2);"] # [doc = ""] # [doc = "     // Shift left: ret <- x1 <- x2 <- next"] # [doc = "     let ret = *x1;"] # [doc = "     *x1 = *x2;"] # [doc = "     *x2 = next;"] # [doc = ""] # [doc = "     // If addition has saturated at the maximum, we are finished"] # [doc = "     if ret == *x1 && ret > 1 {"] # [doc = "         None"] # [doc = "     } else {"] # [doc = "         Some(ret)"] # [doc = "     }"] # [doc = " });"] # [doc = ""] # [doc = " itertools::assert_equal(fibonacci.by_ref().take(8),"] # [doc = "                         vec![1, 1, 2, 3, 5, 8, 13, 21]);"] # [doc = " assert_eq!(fibonacci.last(), Some(2_971_215_073))"] # [doc = " ```"] # [deprecated (note = "Use [std::iter::from_fn](https://doc.rust-lang.org/std/iter/fn.from_fn.html) instead" , since = "0.13.0")] pub fn unfold < A , St , F > (initial_state : St , f : F) -> Unfold < St , F > where F : FnMut (& mut St) -> Option < A > , { Unfold { f , state : initial_state , } }
    };
}

unfold!();