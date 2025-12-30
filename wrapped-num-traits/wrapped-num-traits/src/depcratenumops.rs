// Generated macro for NumOps (trait)
macro_rules! DepcrateNumOps {
() => {
// Module: crate
// Provides: {"NumOps"}
// Dependencies: {}
# [doc = " Generic trait for types implementing basic numeric operations"] # [doc = ""] # [doc = " This is automatically implemented for types which implement the operators."] pub trait NumOps < Rhs = Self , Output = Self > : Add < Rhs , Output = Output > + Sub < Rhs , Output = Output > + Mul < Rhs , Output = Output > + Div < Rhs , Output = Output > + Rem < Rhs , Output = Output > { }
};
}
