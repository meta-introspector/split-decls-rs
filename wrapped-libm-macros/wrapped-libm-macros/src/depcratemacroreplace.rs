// Generated macro for MacroReplace (struct)
macro_rules! DepcrateMacroReplace {
() => {
// Module: crate
// Provides: {"MacroReplace"}
// Dependencies: {}
# [doc = " Visitor to replace \"magic\" identifiers that we allow: `MACRO_FN_NAME` and"] # [doc = " `MACRO_FN_NAME_NORMALIZED`."] struct MacroReplace { fn_name : & 'static str , # [doc = " Remove the trailing `f` or `f128` to make"] norm_name : String , error : Option < syn :: Error > , }
};
}
