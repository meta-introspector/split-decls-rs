// Generated macro for impl_157 (impl)
macro_rules! Depcrate_posimpl_157 {
() => {
// Module: crate::pos
// Provides: {"impl_157"}
// Dependencies: {}
impl < T > Positioned < T > { # [doc = " Create a new positioned node from the node and its position."] # [must_use] pub const fn new (node : T , pos : Pos) -> Positioned < T > { Positioned { pos , node } } # [doc = " Get the inner node."] # [doc = ""] # [doc = " This is most useful in callback chains where `Positioned::into_inner` is"] # [doc = " easier to read than `|positioned| positioned.node`."] # [inline] pub fn into_inner (self) -> T { self . node } # [doc = " Create a new positioned node with the same position as this one."] # [must_use] pub fn position_node < U > (& self , other : U) -> Positioned < U > { Positioned :: new (other , self . pos) } # [doc = " Map the inner value of this positioned node."] # [must_use] pub fn map < U > (self , f : impl FnOnce (T) -> U) -> Positioned < U > { Positioned :: new (f (self . node) , self . pos) } }
};
}
