// Generated macro for TypeInfo (struct)
macro_rules! Depcrate_typesTypeInfo {
() => {
// Module: crate::types
// Provides: {"TypeInfo"}
// Dependencies: {}
# [derive (Default , Clone , Copy , Debug)] pub struct TypeInfo { # [doc = " Whether this type is ever used (transitively) within the"] # [doc = " parameter of an imported function."] # [doc = ""] # [doc = " This means that it's used in a context where ownership isn't"] # [doc = " relinquished."] pub borrowed : bool , # [doc = " Whether this type is ever used (transitively) within the"] # [doc = " parameter or result of an export, or the result of an import."] # [doc = ""] # [doc = " This means that it's used in a context where ownership is required and"] # [doc = " memory management is necessary."] pub owned : bool , # [doc = " Whether this type is ever used (transitively) within the"] # [doc = " error case in the result of a function."] pub error : bool , # [doc = " Whether this type (transitively) has a list (or string)."] pub has_list : bool , # [doc = " Whether this type (transitively) has a tuple."] pub has_tuple : bool , # [doc = " Whether this type (transitively) has a resource (or handle)."] pub has_resource : bool , # [doc = " Whether this type (transitively) has a borrow handle."] pub has_borrow_handle : bool , # [doc = " Whether this type (transitively) has an own handle."] pub has_own_handle : bool , }
};
}
