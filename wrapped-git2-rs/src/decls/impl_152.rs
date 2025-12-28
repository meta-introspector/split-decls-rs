macro_rules! deps {
    () => {
        StringArray!();
        Iter!();
        IterBytes!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl StringArray { # [doc = " Returns None if the i'th string is not utf8 or if i is out of bounds."] pub fn get (& self , i : usize) -> Option < & str > { self . get_bytes (i) . and_then (| s | str :: from_utf8 (s) . ok ()) } # [doc = " Returns None if `i` is out of bounds."] pub fn get_bytes (& self , i : usize) -> Option < & [u8] > { if i < self . raw . count as usize { unsafe { let ptr = * self . raw . strings . add (i) as * const _ ; Some (crate :: opt_bytes (self , ptr) . unwrap ()) } } else { None } } # [doc = " Returns an iterator over the strings contained within this array."] # [doc = ""] # [doc = " The iterator yields `Option<&str>` as it is unknown whether the contents"] # [doc = " are utf-8 or not."] pub fn iter (& self) -> Iter < '_ > { Iter { range : 0 .. self . len () , arr : self , } } # [doc = " Returns an iterator over the strings contained within this array,"] # [doc = " yielding byte slices."] pub fn iter_bytes (& self) -> IterBytes < '_ > { IterBytes { range : 0 .. self . len () , arr : self , } } # [doc = " Returns the number of strings in this array."] pub fn len (& self) -> usize { self . raw . count as usize } # [doc = " Return `true` if this array is empty."] pub fn is_empty (& self) -> bool { self . len () == 0 } }
    };
}

impl_152!();