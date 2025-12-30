// Generated macro for impl_85 (impl)
macro_rules! Depcrate_const_choiceimpl_85 {
() => {
// Module: crate::const_choice
// Provides: {"impl_85"}
// Dependencies: {}
impl < const LIMBS : usize > ConstCtOption < (Uint < LIMBS > , Uint < LIMBS >) > { # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by"] # [doc = " `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> (Uint < LIMBS > , Uint < LIMBS >) { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } }
};
}
