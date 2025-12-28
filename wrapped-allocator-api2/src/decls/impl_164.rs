macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl < T , I : SliceIndex < [T] > , A : Allocator > IndexMut < I > for Vec < T , A > { # [inline (always)] fn index_mut (& mut self , index : I) -> & mut Self :: Output { IndexMut :: index_mut (& mut * * self , index) } }
    };
}

impl_164!();