macro_rules! deps {
    () => {
        Pos!();
        HashValue!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Pos { fn new (index : usize , hash : HashValue) -> Self { Self { nz : unsafe { NonZeroU32 :: new_unchecked (((u32 :: from (hash . 0) << 16) + index as u32) . wrapping_add (1) ,) } , } } fn hash (& self) -> HashValue { HashValue ((self . nz . get () . wrapping_sub (1) >> 16) as u16) } fn index (& self) -> usize { self . nz . get () . wrapping_sub (1) as u16 as usize } }
    };
}

impl_84!();