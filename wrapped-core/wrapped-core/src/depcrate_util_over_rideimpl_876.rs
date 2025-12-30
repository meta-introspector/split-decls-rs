// Generated macro for impl_876 (impl)
macro_rules! Depcrate_util_over_rideimpl_876 {
() => {
// Module: crate::util::over_ride
// Provides: {"impl_876"}
// Dependencies: {}
impl < T > Override < T > { # [doc = " Converts from `Override<T>` to `Override<&T>`."] # [doc = ""] # [doc = " Produces a new `Override`, containing a reference into the original, leaving the original in place."] pub fn as_ref (& self) -> Override < & T > { match * self { Inherit => Inherit , Explicit (ref val) => Explicit (val) , } } # [doc = " Converts from `Override<T>` to `Override<&mut T>`."] # [doc = ""] # [doc = " Produces a new `Override`, containing a mutable reference into the original."] pub fn as_mut (& mut self) -> Override < & mut T > { match * self { Inherit => Inherit , Explicit (ref mut val) => Explicit (val) , } } # [doc = " Returns `true` if the override is an `Explicit` value."] pub fn is_explicit (& self) -> bool { match * self { Inherit => false , Explicit (_) => true , } } # [doc = " Converts from `Override<T>` to `Option<T>`."] pub fn explicit (self) -> Option < T > { match self { Inherit => None , Explicit (val) => Some (val) , } } # [doc = " Unwraps an override, yielding the content of an `Explicit`. Otherwise, it returns `optb`."] pub fn unwrap_or (self , optb : T) -> T { match self { Inherit => optb , Explicit (val) => val , } } # [doc = " Unwraps an override, yielding the content of an `Explicit`. Otherwise, it calls `op`."] pub fn unwrap_or_else < F > (self , op : F) -> T where F : FnOnce () -> T , { match self { Inherit => op () , Explicit (val) => val , } } }
};
}
