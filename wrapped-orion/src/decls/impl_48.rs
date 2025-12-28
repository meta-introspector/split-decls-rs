macro_rules! deps {
    () => {
        U32x4!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl core :: ops :: BitXor for U32x4 { type Output = Self ; fn bitxor (self , _rhs : Self) -> Self :: Output { Self (self . 0 ^ _rhs . 0 , self . 1 ^ _rhs . 1 , self . 2 ^ _rhs . 2 , self . 3 ^ _rhs . 3 ,) } }
    };
}

impl_48!()