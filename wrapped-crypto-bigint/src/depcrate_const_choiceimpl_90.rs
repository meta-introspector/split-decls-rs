// Generated macro for impl_90 (impl)
macro_rules! Depcrate_const_choiceimpl_90 {
() => {
// Module: crate::const_choice
// Provides: {"impl_90"}
// Dependencies: {}
impl < const LIMBS : usize > ConstCtOption < OddInt < LIMBS > > { # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by"] # [doc = " `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> OddInt < LIMBS > { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } }
};
}
