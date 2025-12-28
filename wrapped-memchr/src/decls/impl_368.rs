macro_rules! deps {
    () => {
        FindIter!();
    };
}

macro_rules! impl_368 {
    () => {
        deps!();
        impl < 'h , 'n > Iterator for FindIter < 'h , 'n > { type Item = usize ; fn next (& mut self) -> Option < usize > { let needle = self . finder . needle () ; let haystack = self . haystack . get (self . pos ..) ? ; let idx = self . finder . searcher . find (& mut self . prestate , haystack , needle) ? ; let pos = self . pos + idx ; self . pos = pos + needle . len () . max (1) ; Some (pos) } fn size_hint (& self) -> (usize , Option < usize >) { match self . haystack . len () . checked_sub (self . pos) { None => (0 , Some (0)) , Some (haystack_len) => match self . finder . needle () . len () { 0 => (haystack_len . saturating_add (1) , haystack_len . checked_add (1) ,) , needle_len => (0 , Some (haystack_len / needle_len)) , } , } } }
    };
}

impl_368!()