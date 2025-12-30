// Generated macro for DiffMode (enum)
macro_rules! Depcrate_expand_autodiff_attrsDiffMode {
() => {
// Module: crate::expand::autodiff_attrs
// Provides: {"DiffMode"}
// Dependencies: {}
# [doc = " Forward and Reverse Mode are well known names for automatic differentiation implementations."] # [doc = " Enzyme does support both, but with different semantics, see DiffActivity. The First variants"] # [doc = " are a hack to support higher order derivatives. We need to compute first order derivatives"] # [doc = " before we compute second order derivatives, otherwise we would differentiate our placeholder"] # [doc = " functions. The proper solution is to recognize and resolve this DAG of autodiff invocations,"] # [doc = " as it's already done in the C++ and Julia frontend of Enzyme."] # [doc = ""] # [doc = " Documentation for using [reverse](https://enzyme.mit.edu/rust/rev.html) and"] # [doc = " [forward](https://enzyme.mit.edu/rust/fwd.html) mode is available online."] # [derive (Clone , Copy , Eq , PartialEq , Encodable , Decodable , Debug , HashStable_Generic)] pub enum DiffMode { # [doc = " No autodiff is applied (used during error handling)."] Error , # [doc = " The primal function which we will differentiate."] Source , # [doc = " The target function, to be created using forward mode AD."] Forward , # [doc = " The target function, to be created using reverse mode AD."] Reverse , }
};
}
