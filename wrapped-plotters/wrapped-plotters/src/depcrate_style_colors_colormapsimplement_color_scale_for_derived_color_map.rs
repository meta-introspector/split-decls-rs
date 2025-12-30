// Generated macro for implement_color_scale_for_derived_color_map (macro)
macro_rules! Depcrate_style_colors_colormapsimplement_color_scale_for_derived_color_map {
() => {
// Module: crate::style::colors::colormaps
// Provides: {"implement_color_scale_for_derived_color_map"}
// Dependencies: {}
macro_rules ! implement_color_scale_for_derived_color_map { ($ ($ color_type : ident) ,+) => { $ (impl < FloatType : num_traits :: Float + num_traits :: FromPrimitive + num_traits :: ToPrimitive > ColorMap <$ color_type , FloatType > for DerivedColorMap <$ color_type > { fn get_color_normalized (& self , h : FloatType , min : FloatType , max : FloatType) -> $ color_type { let (relative_difference , index_lower , index_upper) = calculate_relative_difference_index_lower_upper (h , min , max , self . colors . len ()) ; $ crate :: calculate_new_color_value ! (relative_difference , self . colors , index_upper , index_lower , $ color_type) } }) + } }
};
}
