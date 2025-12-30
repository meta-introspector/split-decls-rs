// Generated macro for DecapsulationKey (struct)
macro_rules! Depcrate_hazardous_kem_ml_kem_mlkem512DecapsulationKey {
() => {
// Module: crate::hazardous::kem::ml_kem::mlkem512
// Provides: {"DecapsulationKey"}
// Dependencies: {}
# [derive (Debug , PartialEq)] # [doc = " A type to represent the `DecapsulationKey` that ML-KEM-512 produces."] pub struct DecapsulationKey { pub (crate) value : DecapKey < 2 , 800 , 1632 , MlKem512Internal > , pub (crate) cached_ek : EncapsulationKey , }
};
}
