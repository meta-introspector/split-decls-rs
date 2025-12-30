// Generated macro for declare_result_enum (function)
macro_rules! Depcrate_selectdeclare_result_enum {
() => {
// Module: crate::select
// Provides: {"declare_result_enum"}
// Dependencies: {}
fn declare_result_enum (result_ident : Ident , variants : usize , complete : bool , span : Span ,) -> (Vec < Ident > , syn :: ItemEnum) { let variant_names : Vec < Ident > = (0 .. variants) . map (| num | format_ident ! ("_{}" , num , span = span)) . collect () ; let type_parameters = & variant_names ; let variants = & variant_names ; let complete_variant = if complete { Some (quote ! (Complete)) } else { None } ; let enum_item = parse_quote ! { enum # result_ident <# (# type_parameters ,) *> { # (# variants (# type_parameters) ,) * # complete_variant } } ; (variant_names , enum_item) }
};
}
