macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < T , A : Allocator > ops :: Deref for Vec < T , A > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & [T] { unsafe { slice :: from_raw_parts (self . as_ptr () , self . len) } } }
    };
}

impl_159!();