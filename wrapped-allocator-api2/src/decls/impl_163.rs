macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < T , I : SliceIndex < [T] > , A : Allocator > Index < I > for Vec < T , A > { type Output = I :: Output ; # [inline (always)] fn index (& self , index : I) -> & Self :: Output { Index :: index (& * * self , index) } }
    };
}

impl_163!()