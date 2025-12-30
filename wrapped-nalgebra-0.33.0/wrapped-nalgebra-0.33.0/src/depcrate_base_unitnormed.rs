// Generated macro for Normed (trait)
macro_rules! Depcrate_base_unitNormed {
() => {
// Module: crate::base::unit
// Provides: {"Normed"}
// Dependencies: {}
# [doc = " Trait implemented by entities scan be be normalized and put in an `Unit` struct."] pub trait Normed { # [doc = " The type of the norm."] type Norm : SimdRealField ; # [doc = " Computes the norm."] fn norm (& self) -> Self :: Norm ; # [doc = " Computes the squared norm."] fn norm_squared (& self) -> Self :: Norm ; # [doc = " Multiply `self` by n."] fn scale_mut (& mut self , n : Self :: Norm) ; # [doc = " Divides `self` by n."] fn unscale_mut (& mut self , n : Self :: Norm) ; }
};
}
