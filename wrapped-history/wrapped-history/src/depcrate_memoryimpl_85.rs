// Generated macro for impl_85 (impl)
macro_rules! Depcrate_memoryimpl_85 {
() => {
// Module: crate::memory
// Provides: {"impl_85"}
// Dependencies: {}
impl LocationStack { fn current (& self) -> Location { self . current . clone () } fn len (& self) -> usize { self . prev . len () + self . next . len () + 1 } fn go (& mut self , delta : isize) { match delta . cmp (& 0) { Ordering :: Greater => { for _i in 0 .. delta { if let Some (mut m) = self . next . pop_front () { std :: mem :: swap (& mut m , & mut self . current) ; self . prev . push (m) ; } } } Ordering :: Less => { for _i in 0 .. - delta { if let Some (mut m) = self . prev . pop () { std :: mem :: swap (& mut m , & mut self . current) ; self . next . push_front (m) ; } } } Ordering :: Equal => { } } } fn push (& mut self , mut location : Location) { std :: mem :: swap (& mut location , & mut self . current) ; self . prev . push (location) ; self . next . clear () ; } fn replace (& mut self , location : Location) { self . current = location ; } }
};
}
