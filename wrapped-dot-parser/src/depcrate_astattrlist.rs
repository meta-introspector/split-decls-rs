// Generated macro for AttrList (struct)
macro_rules! Depcrate_astAttrList {
() => {
// Module: crate::ast
// Provides: {"AttrList"}
// Dependencies: {}
# [doc = " A list of `AList`s, i.e. a list of list of attributes. This (strange)"] # [doc = " indirection is induced by the grammar. This structure corresponds to the"] # [doc = " `attr_list` non-terminal of the grammar."] # [doc = ""] # [doc = " Notice methods `flatten` and `flatten_ref` to remove the indirection."] # [derive (Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub struct AttrList < A > { # [doc = " The list of `AList`s."] pub elems : Vec < AList < A > > , }
};
}
