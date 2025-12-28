macro_rules! deps {
    () => {
        Size!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl Sub for Size { type Output = Size ; # [inline] fn sub (self , other : Size) -> Size { Size :: from_bytes (self . bytes () . checked_sub (other . bytes ()) . unwrap_or_else (| | { panic ! ("Size::sub: {} - {} would result in negative size" , self . bytes () , other . bytes ()) })) } }
    };
}

impl_72!();