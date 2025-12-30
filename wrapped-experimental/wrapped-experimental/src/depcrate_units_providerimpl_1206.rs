// Generated macro for impl_1206 (impl)
macro_rules! Depcrate_units_providerimpl_1206 {
() => {
// Module: crate::units::provider
// Provides: {"impl_1206"}
// Dependencies: {}
impl ConversionInfoULE { # [doc = " Extracts the conversion factor as [`super::ratio::IcuRatio`]."] pub (crate) fn factor_as_ratio (& self) -> IcuRatio { let sign : num_bigint :: Sign = Sign :: from_unaligned (self . factor_sign) . into () ; IcuRatio :: from_big_ints (BigInt :: from_bytes_le (sign , self . factor_num () . as_ule_slice ()) , BigInt :: from_bytes_le (num_bigint :: Sign :: Plus , self . factor_den () . as_ule_slice ()) ,) } # [doc = " Extracts the offset as [`super::ratio::IcuRatio`]."] pub (crate) fn offset_as_ratio (& self) -> IcuRatio { let sign : num_bigint :: Sign = Sign :: from_unaligned (self . offset_sign) . into () ; IcuRatio :: from_big_ints (BigInt :: from_bytes_le (sign , self . offset_num () . as_ule_slice ()) , BigInt :: from_bytes_le (num_bigint :: Sign :: Plus , self . offset_den () . as_ule_slice ()) ,) } }
};
}
