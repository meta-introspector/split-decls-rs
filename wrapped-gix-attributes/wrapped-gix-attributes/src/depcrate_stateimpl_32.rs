// Generated macro for impl_32 (impl)
macro_rules! Depcrate_stateimpl_32 {
() => {
// Module: crate::state
// Provides: {"impl_32"}
// Dependencies: {}
# [doc = " Access"] impl StateRef < '_ > { # [doc = " Return `true` if the associated attribute was set to be unspecified using the `!attr` prefix or it wasn't mentioned."] pub fn is_unspecified (& self) -> bool { matches ! (self , StateRef :: Unspecified) } # [doc = " Return `true` if the associated attribute was set with `attr`. Note that this will also be `true` if a value is assigned."] pub fn is_set (& self) -> bool { matches ! (self , StateRef :: Set | StateRef :: Value (_)) } # [doc = " Return `true` if the associated attribute was set with `-attr` to specifically remove it."] pub fn is_unset (& self) -> bool { matches ! (self , StateRef :: Unset) } # [doc = " Attempt to obtain the string value of this state, or return `None` if there is no such value."] pub fn as_bstr (& self) -> Option < & BStr > { match self { StateRef :: Value (v) => Some (v . as_bstr ()) , _ => None , } } }
};
}
