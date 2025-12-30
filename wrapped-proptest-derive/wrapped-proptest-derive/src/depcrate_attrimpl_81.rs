// Generated macro for impl_81 (impl)
macro_rules! Depcrate_attrimpl_81 {
() => {
// Module: crate::attr
// Provides: {"impl_81"}
// Dependencies: {}
impl ParamsMode { # [doc = " Returns `true` iff the mode was explicitly set."] pub fn is_set (& self) -> bool { if let ParamsMode :: Passthrough = * self { false } else { true } } # [doc = " Converts the mode to an `Option` of an `Option` of a type"] # [doc = " where the outer `Option` is `None` iff the mode wasn't set"] # [doc = " and the inner `Option` is `None` iff the mode was `Default`."] pub fn into_option (self) -> Option < Option < Type > > { use self :: ParamsMode :: * ; match self { Passthrough => None , Specified (ty) => Some (Some (ty)) , Default => Some (None) , } } }
};
}
