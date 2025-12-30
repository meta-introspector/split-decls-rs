// Generated macro for impl_135 (impl)
macro_rules! Depcrate_astimpl_135 {
() => {
// Module: crate::ast
// Provides: {"impl_135"}
// Dependencies: {}
impl TypeModifiers { # [doc = " Wraps these [`TypeModifiers`] into the provided [`TypeModifier`]."] fn wrap (& mut self , modifier : TypeModifier) { * self = match (mem :: take (self) , modifier) { (Self :: Static (& []) , TypeModifier :: NonNull) => Self :: Static (& [TypeModifier :: NonNull]) , (Self :: Static (& []) , TypeModifier :: List (None)) => { Self :: Static (& [TypeModifier :: List (None)]) } (Self :: Static (& [TypeModifier :: NonNull]) , TypeModifier :: List (None)) => { Self :: Static (& [TypeModifier :: NonNull , TypeModifier :: List (None)]) } (Self :: Static (s) , modifier) => { let mut vec : Vec < _ > = s . to_vec () ; vec . push (modifier) ; Self :: Dynamic (vec . into_boxed_slice ()) } (Self :: Dynamic (s) , modifier) => { let mut vec = s . into_vec () ; vec . push (modifier) ; Self :: Dynamic (vec . into_boxed_slice ()) } } ; } # [doc = " Removes the last [`TypeModifier`] from these [`TypeModifiers`], if there is any."] fn pop (& mut self) { * self = match mem :: take (self) { Self :: Static (s) => Self :: Static (& s [.. s . len () - 1]) , Self :: Dynamic (s) if s . len () == 1 => Self :: Static (& []) , Self :: Dynamic (s) => { let mut vec = s . into_vec () ; vec . pop () ; Self :: Dynamic (vec . into_boxed_slice ()) } } } }
};
}
