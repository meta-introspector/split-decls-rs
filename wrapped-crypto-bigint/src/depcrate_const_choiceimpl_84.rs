// Generated macro for impl_84 (impl)
macro_rules! Depcrate_const_choiceimpl_84 {
() => {
// Module: crate::const_choice
// Provides: {"impl_84"}
// Dependencies: {}
impl < const LIMBS : usize > ConstCtOption < Uint < LIMBS > > { # [doc = " This returns the underlying value if it is `Some` or the provided value otherwise."] # [inline] pub const fn unwrap_or (self , def : Uint < LIMBS >) -> Uint < LIMBS > { Uint :: select (& def , & self . value , self . is_some) } # [doc = " Returns the contained value, consuming the `self` value."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the value is none with a custom panic message provided by"] # [doc = " `msg`."] # [inline] # [track_caller] pub const fn expect (self , msg : & str) -> Uint < LIMBS > { assert ! (self . is_some . is_true_vartime () , "{}" , msg) ; self . value } # [doc = " Returns the contained value, interpreting the underlying [`Uint`] value as an [`Int`]."] # [inline] pub const fn as_int (& self) -> ConstCtOption < Int < LIMBS > > { ConstCtOption :: new (* self . value . as_int () , self . is_some) } }
};
}
