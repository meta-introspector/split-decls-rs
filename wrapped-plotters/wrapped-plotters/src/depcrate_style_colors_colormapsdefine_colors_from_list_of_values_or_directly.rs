// Generated macro for define_colors_from_list_of_values_or_directly (macro)
macro_rules! Depcrate_style_colors_colormapsdefine_colors_from_list_of_values_or_directly {
() => {
// Module: crate::style::colors::colormaps
// Provides: {"define_colors_from_list_of_values_or_directly"}
// Dependencies: {}
# [macro_export] # [doc (hidden)] # [doc = " Converts a given color identifier and a sequence of colors to an array of them."] macro_rules ! define_colors_from_list_of_values_or_directly { ($ color_type : ident , $ (($ ($ color_value : expr) ,+)) ,+) => { [$ ($ color_type ($ ($ color_value) ,+)) ,+] } ; ($ ($ color_complete : tt) ,+) => { [$ ($ color_complete) ,+] } ; }
};
}
