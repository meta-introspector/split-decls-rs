macro_rules! deps {
    () => {
        StateID!();
        SparseTransitions!();
        Unit!();
    };
}

macro_rules! impl_522 {
    () => {
        deps!();
        impl SparseTransitions { # [doc = " This follows the matching transition for a particular byte."] # [doc = ""] # [doc = " The matching transition is found by looking for a matching byte"] # [doc = " range (there is at most one) corresponding to the position `at` in"] # [doc = " `haystack`."] # [doc = ""] # [doc = " If `at >= haystack.len()`, then this returns `None`."] # [inline] pub fn matches (& self , haystack : & [u8] , at : usize) -> Option < StateID > { haystack . get (at) . and_then (| & b | self . matches_byte (b)) } # [doc = " This follows the matching transition for any member of the alphabet."] # [doc = ""] # [doc = " The matching transition is found by looking for a matching byte"] # [doc = " range (there is at most one) corresponding to the position `at` in"] # [doc = " `haystack`. If the given alphabet unit is [`EOI`](alphabet::Unit::eoi),"] # [doc = " then this always returns `None`."] # [inline] pub (crate) fn matches_unit (& self , unit : alphabet :: Unit ,) -> Option < StateID > { unit . as_u8 () . and_then (| byte | self . matches_byte (byte)) } # [doc = " This follows the matching transition for a particular byte."] # [doc = ""] # [doc = " The matching transition is found by looking for a matching byte range"] # [doc = " (there is at most one) corresponding to the byte given."] # [inline] pub fn matches_byte (& self , byte : u8) -> Option < StateID > { for t in self . transitions . iter () { if t . start > byte { break ; } else if t . matches_byte (byte) { return Some (t . next) ; } } None } }
    };
}

impl_522!();