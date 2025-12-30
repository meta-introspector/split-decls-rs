// Generated macro for impl_92 (impl)
macro_rules! Depcrate_const_choiceimpl_92 {
() => {
// Module: crate::const_choice
// Provides: {"impl_92"}
// Dependencies: {}
impl < const LIMBS : usize > ConstCtOption < SafeGcdInverter < LIMBS > > { # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by"] # [doc = " `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> SafeGcdInverter < LIMBS > { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } }
};
}
