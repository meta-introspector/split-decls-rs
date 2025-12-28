macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'a , T , Slice : ? Sized > From < & 'a Slice > for & 'a RevSlice < T > where Slice : AsRef < [T] > , { fn from (slc : & 'a Slice) -> Self { unsafe { transmute (slc . as_ref ()) } } }
    };
}

impl_54!()