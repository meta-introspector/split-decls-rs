macro_rules! deps {
    () => {
        Drain!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < T > Drop for Drain < '_ , T > { fn drop (& mut self) { if self . deque . len () != self . orig_len - self . range . len () { assert_eq ! (self . deque . len () , self . orig_len) ; self . deque . drain (self . range . clone ()) ; } } }
    };
}

impl_107!()