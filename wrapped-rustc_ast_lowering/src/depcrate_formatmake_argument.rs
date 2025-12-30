// Generated macro for make_argument (function)
macro_rules! Depcrate_formatmake_argument {
() => {
// Module: crate::format
// Provides: {"make_argument"}
// Dependencies: {}
# [doc = " Generate a hir expression representing an argument to a format_args invocation."] # [doc = ""] # [doc = " Generates:"] # [doc = ""] # [doc = " ```text"] # [doc = "     <core::fmt::Argument>::new_…(arg)"] # [doc = " ```"] fn make_argument < 'hir > (ctx : & mut LoweringContext < '_ , 'hir > , sp : Span , arg : & 'hir hir :: Expr < 'hir > , ty : ArgumentType ,) -> hir :: Expr < 'hir > { use ArgumentType :: * ; use FormatTrait :: * ; let new_fn = ctx . arena . alloc (ctx . expr_lang_item_type_relative (sp , hir :: LangItem :: FormatArgument , match ty { Format (Display) => sym :: new_display , Format (Debug) => match ctx . tcx . sess . opts . unstable_opts . fmt_debug { FmtDebug :: Full | FmtDebug :: Shallow => sym :: new_debug , FmtDebug :: None => sym :: new_debug_noop , } , Format (LowerExp) => sym :: new_lower_exp , Format (UpperExp) => sym :: new_upper_exp , Format (Octal) => sym :: new_octal , Format (Pointer) => sym :: new_pointer , Format (Binary) => sym :: new_binary , Format (LowerHex) => sym :: new_lower_hex , Format (UpperHex) => sym :: new_upper_hex , Usize => sym :: from_usize , } ,)) ; ctx . expr_call_mut (sp , new_fn , std :: slice :: from_ref (arg)) }
};
}
