// Generated macro for impl_91 (impl)
macro_rules! Depcrate_const_choiceimpl_91 {
() => {
// Module: crate::const_choice
// Provides: {"impl_91"}
// Dependencies: {}
impl ConstCtOption < NonZero < Limb > > { # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by"] # [doc = " `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> NonZero < Limb > { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } }
};
}
