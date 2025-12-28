macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < 'a , T , Slice : ? Sized > From < & 'a mut Slice > for & 'a mut RevSlice < T > where Slice : AsMut < [T] > , { fn from (slc : & 'a mut Slice) -> Self { unsafe { transmute (slc . as_mut ()) } } }
    };
}

impl_55!();