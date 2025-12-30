// Generated macro for impl_7082 (impl)
macro_rules! Depcrate_methods_wrong_self_conventionimpl_7082 {
() => {
// Module: crate::methods::wrong_self_convention
// Provides: {"impl_7082"}
// Dependencies: {}
impl fmt :: Display for Convention { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { Self :: Eq (this) => write ! (f , "`{this}`") , Self :: StartsWith (this) => write ! (f , "`{this}*`") , Self :: EndsWith (this) => write ! (f , "`*{this}`") , Self :: NotEndsWith (this) => write ! (f , "`~{this}`") , Self :: IsSelfTypeCopy (is_true) => { write ! (f , "`self` type is{} `Copy`" , if is_true { "" } else { " not" }) } , Self :: ImplementsTrait (is_true) => { let (negation , s_suffix) = if is_true { ("" , "s") } else { (" does not" , "") } ; write ! (f , "method{negation} implement{s_suffix} a trait") } , Self :: IsTraitItem (is_true) => { let suffix = if is_true { " is" } else { " is not" } ; write ! (f , "method{suffix} a trait item") } , } } }
};
}
