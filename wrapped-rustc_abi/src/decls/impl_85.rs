macro_rules! deps {
    () => {
        AbiAlign!();
        Align!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl AbiAlign { # [inline] pub fn new (align : Align) -> AbiAlign { AbiAlign { abi : align } } # [inline] pub fn min (self , other : AbiAlign) -> AbiAlign { AbiAlign { abi : self . abi . min (other . abi) } } # [inline] pub fn max (self , other : AbiAlign) -> AbiAlign { AbiAlign { abi : self . abi . max (other . abi) } } }
    };
}

impl_85!()