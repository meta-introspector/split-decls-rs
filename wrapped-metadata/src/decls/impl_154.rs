macro_rules! deps {
    () => {
        STREAM_HEADER!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < const LEN : usize > STREAM_HEADER < LEN > { fn new (offset : u32 , size : u32 , name : & [u8 ; LEN]) -> Self { Self { offset , size , name : * name , } } fn next_offset (& self) -> u32 { self . offset + self . size } }
    };
}

impl_154!()