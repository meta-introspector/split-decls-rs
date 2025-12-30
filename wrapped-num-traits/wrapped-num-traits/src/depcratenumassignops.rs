// Generated macro for NumAssignOps (trait)
macro_rules! DepcrateNumAssignOps {
() => {
// Module: crate
// Provides: {"NumAssignOps"}
// Dependencies: {}
# [doc = " Generic trait for types implementing numeric assignment operators (like `+=`)."] # [doc = ""] # [doc = " This is automatically implemented for types which implement the operators."] pub trait NumAssignOps < Rhs = Self > : AddAssign < Rhs > + SubAssign < Rhs > + MulAssign < Rhs > + DivAssign < Rhs > + RemAssign < Rhs > { }
};
}
