macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < T : Clone > Stack < T > { # [doc = " Creates a new `Stack`."] pub fn new () -> Self { Stack { cache : vec ! [] , popped : vec ! [] , lengths : vec ! [] , } } # [doc = " Returns `true` if the stack is currently empty."] # [allow (dead_code)] pub fn is_empty (& self) -> bool { self . cache . is_empty () } # [doc = " Returns the top-most `&T` in the `Stack`."] pub fn peek (& self) -> Option < & T > { self . cache . last () } # [doc = " Pushes a `T` onto the `Stack`."] pub fn push (& mut self , elem : T) { self . cache . push (elem) ; } # [doc = " Pops the top-most `T` from the `Stack`."] pub fn pop (& mut self) -> Option < T > { let len = self . cache . len () ; let popped = self . cache . pop () ; if let Some (popped) = & popped { if let Some ((_ , remained_count)) = self . lengths . last_mut () { if len == * remained_count { * remained_count -= 1 ; self . popped . push (popped . clone ()) ; } } } popped } # [doc = " Returns the size of the stack"] pub fn len (& self) -> usize { self . cache . len () } # [doc = " Takes a snapshot of the current `Stack`."] pub fn snapshot (& mut self) { self . lengths . push ((self . cache . len () , self . cache . len ())) } # [doc = " The parsing after the last snapshot was successful so clearing it."] pub fn clear_snapshot (& mut self) { if let Some ((len , unpopped)) = self . lengths . pop () { self . popped . truncate (self . popped . len () - (len - unpopped)) ; } } # [doc = " Rewinds the `Stack` to the most recent `snapshot()`. If no `snapshot()` has been taken, this"] # [doc = " function return the stack to its initial state."] pub fn restore (& mut self) { match self . lengths . pop () { Some ((len_stack , remained)) => { if remained < self . cache . len () { self . cache . truncate (remained) ; } if len_stack > remained { let rewind_count = len_stack - remained ; let new_len = self . popped . len () - rewind_count ; let recovered_elements = self . popped . drain (new_len ..) ; self . cache . extend (recovered_elements . rev ()) ; debug_assert_eq ! (self . popped . len () , new_len) ; } } None => { self . cache . clear () ; debug_assert ! (self . popped . is_empty ()) ; debug_assert ! (self . lengths . is_empty ()) ; } } } }
    };
}

impl_166!()