macro_rules! deps {
    () => {
        Size!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl Mul < Size > for u64 { type Output = Size ; # [inline] fn mul (self , size : Size) -> Size { size * self } }
    };
}

impl_73!();