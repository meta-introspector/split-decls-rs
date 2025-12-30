// Generated macro for impl_215 (impl)
macro_rules! Depcrate_optionsimpl_215 {
() => {
// Module: crate::options
// Provides: {"impl_215"}
// Dependencies: {}
# [cfg (all (feature = "serde" , feature = "experimental"))] impl From < TimePrecision > for TimePrecisionSerde { fn from (value : TimePrecision) -> Self { match value { TimePrecision :: Hour => TimePrecisionSerde :: Hour , TimePrecision :: Minute => TimePrecisionSerde :: Minute , TimePrecision :: Second => TimePrecisionSerde :: Second , TimePrecision :: Subsecond (SubsecondDigits :: S1) => TimePrecisionSerde :: Subsecond1 , TimePrecision :: Subsecond (SubsecondDigits :: S2) => TimePrecisionSerde :: Subsecond2 , TimePrecision :: Subsecond (SubsecondDigits :: S3) => TimePrecisionSerde :: Subsecond3 , TimePrecision :: Subsecond (SubsecondDigits :: S4) => TimePrecisionSerde :: Subsecond4 , TimePrecision :: Subsecond (SubsecondDigits :: S5) => TimePrecisionSerde :: Subsecond5 , TimePrecision :: Subsecond (SubsecondDigits :: S6) => TimePrecisionSerde :: Subsecond6 , TimePrecision :: Subsecond (SubsecondDigits :: S7) => TimePrecisionSerde :: Subsecond7 , TimePrecision :: Subsecond (SubsecondDigits :: S8) => TimePrecisionSerde :: Subsecond8 , TimePrecision :: Subsecond (SubsecondDigits :: S9) => TimePrecisionSerde :: Subsecond9 , TimePrecision :: MinuteOptional => TimePrecisionSerde :: MinuteOptional , } } }
};
}
