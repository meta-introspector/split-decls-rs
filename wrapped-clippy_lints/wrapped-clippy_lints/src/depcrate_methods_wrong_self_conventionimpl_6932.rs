// Generated macro for impl_6932 (impl)
macro_rules! Depcrate_methods_wrong_self_conventionimpl_6932 {
() => {
// Module: crate::methods::wrong_self_convention
// Provides: {"impl_6932"}
// Dependencies: {}
impl fmt :: Display for Convention { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { Self :: Eq (this) => format ! ("`{this}`") . fmt (f) , Self :: StartsWith (this) => format ! ("`{this}*`") . fmt (f) , Self :: EndsWith (this) => format ! ("`*{this}`") . fmt (f) , Self :: NotEndsWith (this) => format ! ("`~{this}`") . fmt (f) , Self :: IsSelfTypeCopy (is_true) => { format ! ("`self` type is{} `Copy`" , if is_true { "" } else { " not" }) . fmt (f) } , Self :: ImplementsTrait (is_true) => { let (negation , s_suffix) = if is_true { ("" , "s") } else { (" does not" , "") } ; format ! ("method{negation} implement{s_suffix} a trait") . fmt (f) } , Self :: IsTraitItem (is_true) => { let suffix = if is_true { " is" } else { " is not" } ; format ! ("method{suffix} a trait item") . fmt (f) } , } } }
};
}
