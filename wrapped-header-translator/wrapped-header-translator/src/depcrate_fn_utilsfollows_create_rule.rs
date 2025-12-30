// Generated macro for follows_create_rule (function)
macro_rules! Depcrate_fn_utilsfollows_create_rule {
() => {
// Module: crate::fn_utils
// Provides: {"follows_create_rule"}
// Dependencies: {}
# [doc = " Algorithm described in:"] # [doc = " <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#auditing-of-c-retainable-pointer-interfaces>"] # [doc = ""] # [doc = " > A function obeys the create/copy naming convention if its name"] # [doc = " > contains as a substring:"] # [doc = " > - either “Create” or “Copy” not followed by a lowercase letter, or"] # [doc = " > - either “create” or “copy” not followed by a lowercase letter and"] # [doc = " >   not preceded by any letter, whether uppercase or lowercase."] # [doc = ""] # [doc = " See also Clang's implementation:"] # [doc = " <https://github.com/llvm/llvm-project/blob/llvmorg-19.1.6/clang/lib/Analysis/CocoaConventions.cpp#L97-L145>"] # [doc = " <https://github.com/llvm/llvm-project/blob/llvmorg-19.1.6/clang/lib/Analysis/RetainSummaryManager.cpp>"] pub (crate) fn follows_create_rule (name : & str) -> bool { static RE : LazyLock < Regex > = LazyLock :: new (| | { Regex :: new (r"(Create|Copy)([^a-z]|$)|([^a-zA-Z]|^)(create|copy)([^a-z]|$)") . unwrap () }) ; RE . is_match (name) }
};
}
