macro_rules! deps {
    () => {
        PutBackN!();
    };
}

macro_rules! impl_453 {
    () => {
        deps!();
        impl < I : Iterator > PutBackN < I > { # [doc = " Puts `x` in front of the iterator."] # [doc = ""] # [doc = " The values are yielded in order of the most recently put back"] # [doc = " values first."] # [doc = ""] # [doc = " ```rust"] # [doc = " use itertools::put_back_n;"] # [doc = ""] # [doc = " let mut it = put_back_n(1..5);"] # [doc = " it.next();"] # [doc = " it.put_back(1);"] # [doc = " it.put_back(0);"] # [doc = ""] # [doc = " assert!(itertools::equal(it, 0..5));"] # [doc = " ```"] # [inline] pub fn put_back (& mut self , x : I :: Item) { self . top . push (x) ; } }
    };
}

impl_453!();