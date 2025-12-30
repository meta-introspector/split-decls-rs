// Generated macro for impl_154 (impl)
macro_rules! Depcrate_deriveimpl_154 {
() => {
// Module: crate::derive
// Provides: {"impl_154"}
// Dependencies: {}
impl < C > StratAcc < C > { # [doc = " Construct the given accumulator with"] # [doc = " initial capacity according to `size`."] fn new (size : usize) -> Self { Self { types : Vec :: with_capacity (size) , ctors : Vec :: with_capacity (size) , } } # [doc = " Add the given type and constructor pair to"] # [doc = " the accumulator which is moved and returned."] fn add (mut self , (strat , ctor) : (Strategy , C)) -> Self { self . types . push (strat) ; self . ctors . push (ctor) ; self } # [doc = " Consume the accumulator returning the:"] # [doc = " + sequence of strategies"] # [doc = " + sequence of constructors"] fn consume (self) -> (Vec < Strategy > , Vec < C >) { (self . types , self . ctors) } # [doc = " Returns `true` iff nothing has been accumulated yet."] fn is_empty (& self) -> bool { self . types . is_empty () } }
};
}
