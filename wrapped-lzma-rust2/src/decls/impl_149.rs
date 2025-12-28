macro_rules! deps {
    () => {
        FilterType!();
        Error!();
        Result!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl TryFrom < u64 > for FilterType { type Error = () ; fn try_from (value : u64) -> Result < Self , Self :: Error > { match value { 0x03 => Ok (FilterType :: Delta) , 0x04 => Ok (FilterType :: BcjX86) , 0x05 => Ok (FilterType :: BcjPpc) , 0x06 => Ok (FilterType :: BcjIa64) , 0x07 => Ok (FilterType :: BcjArm) , 0x08 => Ok (FilterType :: BcjArmThumb) , 0x09 => Ok (FilterType :: BcjSparc) , 0x0A => Ok (FilterType :: BcjArm64) , 0x0B => Ok (FilterType :: BcjRiscv) , 0x21 => Ok (FilterType :: Lzma2) , _ => Err (()) , } } }
    };
}

impl_149!()