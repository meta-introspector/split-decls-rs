// Generated macro for method_family (function)
macro_rules! Depcrate___macros_method_familymethod_family {
() => {
// Module: crate::__macros::method_family
// Provides: {"method_family"}
// Dependencies: {}
# [doc = " Determine the constant to specify in `MethodFamily` to get the method"] # [doc = " family type."] # [doc = ""] # [doc = " This is only called with the first part of the selector, as that's enough"] # [doc = " to determine the family, and that way we can emit less code for rustc to"] # [doc = " parse."] # [doc = ""] # [doc = " Examples:"] # [doc = " - `init` in `init`, returns `3`."] # [doc = " - `allocWithZone` in `allocWithZone:`, returns `2`."] # [doc = " - `copyItemAtURL` in `copyItemAtURL:toURL:error:`, returns `4`."] # [doc = " - `convertRect` in `convertRect:fromView:`, returns `6`."] pub const fn method_family (first_selector_part : & str) -> u8 { let first_selector_part = first_selector_part . as_bytes () ; match (in_selector_family (first_selector_part , b"new") , in_selector_family (first_selector_part , b"alloc") , in_selector_family (first_selector_part , b"init") , in_selector_family (first_selector_part , b"copy") , in_selector_family (first_selector_part , b"mutableCopy") ,) { (true , false , false , false , false) => 1 , (false , true , false , false , false) => 2 , (false , false , true , false , false) => 3 , (false , false , false , true , false) => 4 , (false , false , false , false , true) => 5 , (false , false , false , false , false) => 6 , _ => unreachable ! () , } }
};
}
