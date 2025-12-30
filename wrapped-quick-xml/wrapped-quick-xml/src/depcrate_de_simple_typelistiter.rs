// Generated macro for ListIter (struct)
macro_rules! Depcrate_de_simple_typeListIter {
() => {
// Module: crate::de::simple_type
// Provides: {"ListIter"}
// Dependencies: {}
# [doc = " Iterator over string sub-slices delimited by one or several spaces."] # [doc = " Contains decoded value of the `simpleType`."] # [doc = " Iteration ends when list contains `None`."] struct ListIter < 'de , 'a > { # [doc = " If `Some`, contains unconsumed data of the list"] content : Option < Content < 'de , 'a > > , # [doc = " If `true`, `content` in escaped form and should be unescaped before use"] escaped : bool , }
};
}
