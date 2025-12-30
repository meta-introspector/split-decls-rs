// Generated macro for DecapsulationKey (struct)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem1024DecapsulationKey {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem1024
// Provides: {"DecapsulationKey"}
// Dependencies: {}
# [derive (Debug , PartialEq)] # [doc = " A type to represent the `DecapsulationKey` that ML-KEM-1024 produces."] pub struct DecapsulationKey { pub (crate) value : DecapKey < 4 , 1568 , 3168 , MlKem1024Internal > , pub (crate) cached_ek : EncapsulationKey , }
};
}
