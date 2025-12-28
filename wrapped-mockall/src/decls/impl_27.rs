macro_rules! deps {
    () => {
        TimesRange!();
        ExpectedCalls!();
        Times!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        # [doc (hidden)] impl Times { pub fn call (& self) -> Result < () , String > { let count = self . count . fetch_add (1 , Ordering :: Relaxed) + 1 ; if count >= self . range . 0 . end { if self . range . 0 . end == 1 { Err ("should not have been called" . to_owned ()) } else { Err (format ! ("called {} times which is more than the expected {}" , count , self . range . 0 . end - 1)) } } else { Ok (()) } } pub fn any (& mut self) { self . range . 0 = 0 .. usize :: MAX ; } # [doc = " Return how many times this expectation has been called"] pub fn count (& self) -> usize { self . count . load (Ordering :: Relaxed) } # [doc = " Has this expectation already been called the maximum allowed number of"] # [doc = " times?"] pub fn is_done (& self) -> bool { self . count . load (Ordering :: Relaxed) >= self . range . 0 . end - 1 } # [doc = " Is it required that this expectation be called an exact number of times,"] # [doc = " or may it be satisfied by a range of call counts?"] pub fn is_exact (& self) -> bool { (self . range . 0 . end - self . range . 0 . start) == 1 } # [doc = " Has this expectation already been called the expected number of times?"] # [doc = " If not, was it too many or too few?"] pub fn is_satisfied (& self) -> ExpectedCalls { let satisfied_lower_bound = self . count . load (Ordering :: Relaxed) >= self . range . 0 . start ; let satisfied_upper_bound = self . count . load (Ordering :: Relaxed) < self . range . 0 . end ; if satisfied_lower_bound && satisfied_upper_bound { ExpectedCalls :: Satisfied } else if satisfied_lower_bound { ExpectedCalls :: TooMany } else { ExpectedCalls :: TooFew } } # [doc = " The maximum number of times that this expectation must be called"] pub fn maximum (& self) -> usize { self . range . 0 . end - 1 } # [doc = " The minimum number of times that this expectation must be called"] pub fn minimum (& self) -> usize { self . range . 0 . start } # [allow (clippy :: range_plus_one)] pub fn n (& mut self , n : usize) { self . range . 0 = n .. (n + 1) ; } pub fn never (& mut self) { self . range . 0 = 0 .. 1 ; } pub fn range (& mut self , range : Range < usize >) { assert ! (range . end > range . start , "Backwards range") ; self . range . 0 = range ; } pub fn times < T : Into < TimesRange > > (& mut self , t : T) { self . range = t . into () ; } }
    };
}

impl_27!()