macro_rules! deps {
    () => {
        Limb!();
    };
}

macro_rules! nlimbs {
    () => {
        deps!();
        # [doc = " Calculate the number of limbs required to represent the given number of bits."] # [macro_export] macro_rules ! nlimbs { ($ bits : expr) => { u32 :: div_ceil ($ bits , $ crate :: Limb :: BITS) as usize } ; }
    };
}

nlimbs!()