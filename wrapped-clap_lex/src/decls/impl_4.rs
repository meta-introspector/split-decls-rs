macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl < 's > Iterator for Split < 's , '_ > { type Item = & 's OsStr ; fn next (& mut self) -> Option < Self :: Item > { let haystack = self . haystack ? ; if let Some ((first , second)) = haystack . split_once (self . needle) { if ! haystack . is_empty () { debug_assert_ne ! (haystack , second) ; } self . haystack = Some (second) ; Some (first) } else { self . haystack = None ; Some (haystack) } } }
    };
}

impl_4!();