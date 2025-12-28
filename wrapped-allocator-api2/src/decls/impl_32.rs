macro_rules! deps {
    () => {
        Box!();
        Allocator!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T : ? Sized + PartialEq , A : Allocator > PartialEq for Box < T , A > { # [inline (always)] fn eq (& self , other : & Self) -> bool { PartialEq :: eq (& * * self , & * * other) } # [inline (always)] fn ne (& self , other : & Self) -> bool { PartialEq :: ne (& * * self , & * * other) } }
    };
}

impl_32!();