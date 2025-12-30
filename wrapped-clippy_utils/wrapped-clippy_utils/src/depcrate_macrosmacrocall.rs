// Generated macro for MacroCall (struct)
macro_rules! Depcrate_macrosMacroCall {
() => {
// Module: crate::macros
// Provides: {"MacroCall"}
// Dependencies: {}
# [doc = " A macro call, like `vec![1, 2, 3]`."] # [doc = ""] # [doc = " Use `tcx.item_name(macro_call.def_id)` to get the macro name."] # [doc = " Even better is to check if it is a diagnostic item."] # [doc = ""] # [doc = " This structure is similar to `ExpnData` but it precludes desugaring expansions."] # [derive (Debug)] pub struct MacroCall { # [doc = " Macro `DefId`"] pub def_id : DefId , # [doc = " Kind of macro"] pub kind : MacroKind , # [doc = " The expansion produced by the macro call"] pub expn : ExpnId , # [doc = " Span of the macro call site"] pub span : Span , }
};
}
