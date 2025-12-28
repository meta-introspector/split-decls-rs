macro_rules! deps {
    () => {
        BufList!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl < T : Buf > BufList < T > { pub (crate) fn new () -> BufList < T > { BufList { bufs : VecDeque :: new () , } } # [inline] pub (crate) fn push (& mut self , buf : T) { debug_assert ! (buf . has_remaining ()) ; self . bufs . push_back (buf) ; } # [inline] pub (crate) fn bufs_cnt (& self) -> usize { self . bufs . len () } }
    };
}

impl_43!();