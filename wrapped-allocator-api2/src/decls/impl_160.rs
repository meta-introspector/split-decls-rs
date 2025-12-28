macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl < T , A : Allocator > ops :: DerefMut for Vec < T , A > { # [inline (always)] fn deref_mut (& mut self) -> & mut [T] { unsafe { slice :: from_raw_parts_mut (self . as_mut_ptr () , self . len) } } }
    };
}

impl_160!()