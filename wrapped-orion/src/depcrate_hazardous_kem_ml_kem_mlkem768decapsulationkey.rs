// Generated macro for DecapsulationKey (struct)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem768DecapsulationKey {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem768
// Provides: {"DecapsulationKey"}
// Dependencies: {}
# [derive (Debug , PartialEq)] # [doc = " A type to represent the `DecapsulationKey` that ML-KEM-768 produces."] pub struct DecapsulationKey { pub (crate) value : DecapKey < 3 , 1184 , 2400 , MlKem768Internal > , pub (crate) cached_ek : EncapsulationKey , }
};
}
