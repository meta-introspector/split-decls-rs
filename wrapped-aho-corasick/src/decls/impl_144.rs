macro_rules! deps {
    () => {
        Span!();
        Searcher!();
        PatternID!();
        Match!();
    };
}

macro_rules! impl_144 {
    () => {
        deps!();
        impl Searcher { # [doc = " Look for the leftmost occurrence of any pattern in this search in the"] # [doc = " given haystack starting at the given position."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This panics when `haystack[at..].len()` is less than the minimum length"] # [doc = " for this haystack."] # [inline (always)] pub (crate) fn find (& self , haystack : & [u8] , at : usize ,) -> Option < crate :: Match > { assert ! (haystack [at ..] . len () >= self . minimum_len) ; let hayptr = haystack . as_ptr () ; let teddym = unsafe { self . imp . find (hayptr . add (at) , hayptr . add (haystack . len ())) ? } ; let start = teddym . start () . as_usize () . wrapping_sub (hayptr . as_usize ()) ; let end = teddym . end () . as_usize () . wrapping_sub (hayptr . as_usize ()) ; let span = crate :: Span { start , end } ; let pid = crate :: PatternID :: new_unchecked (teddym . pattern () . as_usize ()) ; let m = crate :: Match :: new (pid , span) ; Some (m) } # [doc = " Returns the approximate total amount of heap used by this type, in"] # [doc = " units of bytes."] # [inline (always)] pub (crate) fn memory_usage (& self) -> usize { self . memory_usage } # [doc = " Returns the minimum length, in bytes, that a haystack must be in order"] # [doc = " to use it with this searcher."] # [inline (always)] pub (crate) fn minimum_len (& self) -> usize { self . minimum_len } }
    };
}

impl_144!();