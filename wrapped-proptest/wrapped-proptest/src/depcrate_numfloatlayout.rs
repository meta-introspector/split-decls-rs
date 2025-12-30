// Generated macro for FloatLayout (trait)
macro_rules! Depcrate_numFloatLayout {
() => {
// Module: crate::num
// Provides: {"FloatLayout"}
// Dependencies: {}
trait FloatLayout where StandardUniform : Distribution < Self :: Bits > , { type Bits : Copy ; const SIGN_MASK : Self :: Bits ; const EXP_MASK : Self :: Bits ; const EXP_ZERO : Self :: Bits ; const MANTISSA_MASK : Self :: Bits ; }
};
}
