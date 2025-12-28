macro_rules! deps {
    () => {
        ExtendedFloat!();
        ModeratePathPowers!();
    };
}

macro_rules! impl_395 {
    () => {
        deps!();
        # [doc = " Allow indexing of values without bounds checking"] impl ModeratePathPowers { # [inline] pub fn get_small (& self , index : usize) -> ExtendedFloat { self . small . get_extended_float (index) } # [inline] pub fn get_large (& self , index : usize) -> ExtendedFloat { self . large . get_extended_float (index) } # [inline] pub fn get_small_int (& self , index : usize) -> u64 { self . small_int [index] } }
    };
}

impl_395!();