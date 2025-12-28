macro_rules! BellerophonPowers {
    () => {
        # [doc = " Precalculated powers of base N for the Bellerophon algorithm."] pub struct BellerophonPowers { pub small : & 'static [u64] , pub large : & 'static [u64] , # [doc = " Pre-calculated small powers as 64-bit integers"] pub small_int : & 'static [u64] , pub step : i32 , pub bias : i32 , # [doc = " ceil(log2(radix)) scaled as a multiplier."] pub log2 : i64 , # [doc = " Bitshift for the log2 multiplier."] pub log2_shift : i32 , }
    };
}

BellerophonPowers!();