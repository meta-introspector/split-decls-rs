// Generated macro for impl_259 (impl)
macro_rules! Depcrate_templateimpl_259 {
() => {
// Module: crate::template
// Provides: {"impl_259"}
// Dependencies: {}
impl ValueType { # [doc = " Returns the value type for an undefined variable."] # [inline] # [must_use] pub const fn undefined () -> Self { ValueType :: Undefined } # [doc = " Returns the value type for a string variable."] # [inline] # [must_use] pub const fn string () -> Self { ValueType :: String } # [doc = " Returns the value type for an empty list variable."] # [inline] # [must_use] pub const fn empty_list () -> Self { ValueType :: Undefined } # [doc = " Returns the value type for a nonempty list variable."] # [inline] # [must_use] pub const fn nonempty_list () -> Self { ValueType :: List } # [doc = " Returns the value type for an empty associative array variable."] # [inline] # [must_use] pub const fn empty_assoc () -> Self { ValueType :: Undefined } # [doc = " Returns the value type for a nonempty associative array variable."] # [inline] # [must_use] pub const fn nonempty_assoc () -> Self { ValueType :: Assoc } }
};
}
