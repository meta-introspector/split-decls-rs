macro_rules! deps {
    () => {
        Offset!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Offset { # [doc = " Shift the given `range` according to our offset."] pub fn shifted_range (& self , range : & Range < u32 >) -> Range < u32 > { match self { Offset :: Added (added) => { debug_assert ! (range . start >= * added , "{self:?} {range:?}") ; Range { start : range . start - added , end : range . end - added , } } Offset :: Deleted (deleted) => Range { start : range . start + deleted , end : range . end + deleted , } , } } }
    };
}

impl_11!();