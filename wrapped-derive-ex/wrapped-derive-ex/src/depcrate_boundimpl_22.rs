// Generated macro for impl_22 (impl)
macro_rules! Depcrate_boundimpl_22 {
() => {
// Module: crate::bound
// Provides: {"impl_22"}
// Dependencies: {}
impl Bounds { pub const fn new () -> Self { Self { ty : Vec :: new () , pred : Vec :: new () , default : true , } } pub fn from (bound : & Option < NameArgs < Vec < Bound > > >) -> Self { let mut this = Self :: new () ; if let Some (bound) = bound { this . default = false ; for b in & bound . args { this . push (b . clone ()) ; } } this } fn push (& mut self , bound : Bound) { match bound { Bound :: Type (ty) => self . ty . push (ty) , Bound :: Pred (pred) => self . pred . push (pred) , Bound :: Default (_) => self . default = true , } } }
};
}
