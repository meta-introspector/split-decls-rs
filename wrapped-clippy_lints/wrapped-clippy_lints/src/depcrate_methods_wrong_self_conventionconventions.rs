// Generated macro for CONVENTIONS (const)
macro_rules! Depcrate_methods_wrong_self_conventionCONVENTIONS {
() => {
// Module: crate::methods::wrong_self_convention
// Provides: {"CONVENTIONS"}
// Dependencies: {}
# [rustfmt :: skip] const CONVENTIONS : [(& [Convention] , & [SelfKind]) ; 9] = [(& [Convention :: Eq ("new")] , & [SelfKind :: No]) , (& [Convention :: StartsWith ("as_")] , & [SelfKind :: Ref , SelfKind :: RefMut]) , (& [Convention :: StartsWith ("from_")] , & [SelfKind :: No]) , (& [Convention :: StartsWith ("into_")] , & [SelfKind :: Value]) , (& [Convention :: StartsWith ("is_")] , & [SelfKind :: RefMut , SelfKind :: Ref , SelfKind :: No]) , (& [Convention :: Eq ("to_mut")] , & [SelfKind :: RefMut]) , (& [Convention :: StartsWith ("to_") , Convention :: EndsWith ("_mut")] , & [SelfKind :: RefMut]) , (& [Convention :: StartsWith ("to_") , Convention :: NotEndsWith ("_mut") , Convention :: IsSelfTypeCopy (false) , Convention :: IsTraitItem (false) , Convention :: ImplementsTrait (false)] , & [SelfKind :: Ref]) , (& [Convention :: StartsWith ("to_") , Convention :: NotEndsWith ("_mut") , Convention :: IsSelfTypeCopy (true) , Convention :: IsTraitItem (false) , Convention :: ImplementsTrait (false)] , & [SelfKind :: Value]) ,] ;
};
}
