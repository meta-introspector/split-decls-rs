macro_rules! deps {
    () => {
        BellerophonPowers!();
        ExtendedFloat!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [doc = " Allow indexing of values without bounds checking"] impl BellerophonPowers { # [inline] pub fn get_small (& self , index : usize) -> ExtendedFloat { let mant = self . small [index] ; let exp = (1 - 64) + ((self . log2 * index as i64) >> self . log2_shift) ; ExtendedFloat { mant , exp : exp as i32 , } } # [inline] pub fn get_large (& self , index : usize) -> ExtendedFloat { let mant = self . large [index] ; let biased_e = index as i64 * self . step as i64 - self . bias as i64 ; let exp = (1 - 64) + ((self . log2 * biased_e) >> self . log2_shift) ; ExtendedFloat { mant , exp : exp as i32 , } } # [inline] pub fn get_small_int (& self , index : usize) -> u64 { self . small_int [index] } }
    };
}

impl_8!();