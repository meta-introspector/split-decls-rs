// Generated macro for AnyExt (trait)
macro_rules! Depcrate_value_scalarAnyExt {
() => {
// Module: crate::value::scalar
// Provides: {"AnyExt"}
// Dependencies: {}
# [doc = " Extension of [`Any`] for using its methods directly on the value without `dyn`."] pub trait AnyExt : Any { # [doc = " Returns `true` if the this type is the same as `T`."] # [must_use] fn is < T : Any + ? Sized > (& self) -> bool { TypeId :: of :: < T > () == self . type_id () } # [doc = " Returns [`Some`] reference to this value if it's of type `T`, or [`None`] otherwise."] # [must_use] fn downcast_ref < T : Any > (& self) -> Option < & T > { self . is :: < T > () . then (| | unsafe { & * (ptr :: from_ref (self) as * const T) }) } # [doc = " Returns [`Some`] mutable reference to this value if it's of type `T`, or [`None`] otherwise."] # [must_use] fn downcast_mut < T : Any > (& mut self) -> Option < & mut T > { (TypeId :: of :: < Self > () == TypeId :: of :: < T > ()) . then (| | unsafe { & mut * (ptr :: from_mut (self) as * mut T) }) } }
};
}
