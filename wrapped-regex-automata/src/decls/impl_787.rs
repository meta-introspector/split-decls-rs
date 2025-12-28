macro_rules! deps {
    () => {
        Start!();
    };
}

macro_rules! impl_787 {
    () => {
        deps!();
        impl Start { # [doc = " Return the starting state corresponding to the given integer. If no"] # [doc = " starting state exists for the given integer, then None is returned."] pub (crate) fn from_usize (n : usize) -> Option < Start > { match n { 0 => Some (Start :: NonWordByte) , 1 => Some (Start :: WordByte) , 2 => Some (Start :: Text) , 3 => Some (Start :: LineLF) , 4 => Some (Start :: LineCR) , 5 => Some (Start :: CustomLineTerminator) , _ => None , } } # [doc = " Returns the total number of starting state configurations."] pub (crate) fn len () -> usize { 6 } # [doc = " Return this starting configuration as `u8` integer. It is guaranteed to"] # [doc = " be less than `Start::len()`."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn as_u8 (& self) -> u8 { * self as u8 } # [doc = " Return this starting configuration as a `usize` integer. It is"] # [doc = " guaranteed to be less than `Start::len()`."] # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn as_usize (& self) -> usize { usize :: from (self . as_u8 ()) } }
    };
}

impl_787!()