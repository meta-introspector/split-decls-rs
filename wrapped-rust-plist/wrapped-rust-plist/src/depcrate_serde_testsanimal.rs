// Generated macro for Animal (enum)
macro_rules! Depcrate_serde_testsAnimal {
() => {
// Module: crate::serde_tests
// Provides: {"Animal"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Serialize , Deserialize)] enum Animal { Cow , Dog (DogOuter) , Frog (Result < String , bool > , Option < Vec < f64 > >) , Cat { age : Integer , name : String , firmware : Option < Vec < u8 > > , } , }
};
}
