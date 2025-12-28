macro_rules! deps {
    () => {
        Size!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl Add for Size { type Output = Size ; # [inline] fn add (self , other : Size) -> Size { Size :: from_bytes (self . bytes () . checked_add (other . bytes ()) . unwrap_or_else (| | { panic ! ("Size::add: {} + {} doesn't fit in u64" , self . bytes () , other . bytes ()) })) } }
    };
}

impl_71!()