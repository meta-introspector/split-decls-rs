// Generated macro for impl_12 (impl)
macro_rules! Depcrateimpl_12 {
() => {
// Module: crate
// Provides: {"impl_12"}
// Dependencies: {}
impl < F : ? Sized > FutureWrapper < F > { # [doc = " Get a reference to the inner future."] # [inline] pub fn get_ref (& self) -> & F { & self . inner } # [doc = " Get a mutable reference to the inner future."] # [inline] pub fn get_mut (& mut self) -> & mut F { & mut self . inner } # [doc = " Get a pinned mutable reference to the inner future."] # [inline] pub fn get_pin_mut (self : Pin < & mut Self >) -> Pin < & mut F > { self . project () . inner } # [doc = " Get a pinned reference to the inner future."] # [inline] pub fn get_pin_ref (self : Pin < & Self >) -> Pin < & F > { self . project_ref () . inner } }
};
}
