// Generated macro for impl_1160 (impl)
macro_rules! Depcrate_units_converterimpl_1160 {
() => {
// Module: crate::units::converter
// Provides: {"impl_1160"}
// Dependencies: {}
impl < N > UnitsConverterInner < N > where N : Convertible , { # [doc = " Converts the given value from the input unit to the output unit based on the inner converter type."] fn convert (& self , value : & N) -> N { match self { UnitsConverterInner :: Proportional (converter) => converter . convert (value) , UnitsConverterInner :: Reciprocal (converter) => converter . convert (value) , UnitsConverterInner :: Offset (converter) => converter . convert (value) , } } }
};
}
