macro_rules! deps {
    () => {
        Align!();
        AbiAlign!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl AbiAlign { # [inline] pub fn new (align : Align) -> AbiAlign { AbiAlign { abi : align } } # [inline] pub fn min (self , other : AbiAlign) -> AbiAlign { AbiAlign { abi : self . abi . min (other . abi) } } # [inline] pub fn max (self , other : AbiAlign) -> AbiAlign { AbiAlign { abi : self . abi . max (other . abi) } } }
    };
}

impl_44!()