macro_rules! deps {
    () => {
        Digest!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl Digest { pub const fn bytes (& self) -> [u8 ; 20] { [(self . data [0] >> 24) as u8 , (self . data [0] >> 16) as u8 , (self . data [0] >> 8) as u8 , self . data [0] as u8 , (self . data [1] >> 24) as u8 , (self . data [1] >> 16) as u8 , (self . data [1] >> 8) as u8 , self . data [1] as u8 , (self . data [2] >> 24) as u8 , (self . data [2] >> 16) as u8 , (self . data [2] >> 8) as u8 , self . data [2] as u8 , (self . data [3] >> 24) as u8 , (self . data [3] >> 16) as u8 , (self . data [3] >> 8) as u8 , self . data [3] as u8 , (self . data [4] >> 24) as u8 , (self . data [4] >> 16) as u8 , (self . data [4] >> 8) as u8 , self . data [4] as u8 ,] } }
    };
}

impl_91!()