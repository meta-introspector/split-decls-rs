// Generated macro for impl_114 (impl)
macro_rules! Depcrateimpl_114 {
() => {
// Module: crate
// Provides: {"impl_114"}
// Dependencies: {}
impl Expression { # [doc = " Constructs an expression of the form _n_, where n is a constant real number, not a variable."] pub fn from_constant (v : f64) -> Expression { Expression { terms : Vec :: new () , constant : v } } # [doc = " Constructs an expression from a single term. Forms an expression of the form _n x_"] # [doc = " where n is the coefficient, and x is the variable."] pub fn from_term (term : Term) -> Expression { Expression { terms : vec ! [term] , constant : 0.0 } } # [doc = " General constructor. Each `Term` in `terms` is part of the sum forming the expression, as well as `constant`."] pub fn new (terms : Vec < Term > , constant : f64) -> Expression { Expression { terms : terms , constant : constant } } # [doc = " Mutates this expression by multiplying it by minus one."] pub fn negate (& mut self) { self . constant = - self . constant ; for t in & mut self . terms { * t = - * t ; } } }
};
}
