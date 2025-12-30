// Generated macro for define_units_data_markers (macro)
macro_rules! Depcrate_dimension_provider_units_categorized_display_namesdefine_units_data_markers {
() => {
// Module: crate::dimension::provider::units::categorized_display_names
// Provides: {"define_units_data_markers"}
// Dependencies: {}
macro_rules ! define_units_data_markers { ($ ($ marker : ident , $ doc : literal) ;* $ (;) ?) => { $ (icu_provider :: data_marker ! (# [doc = $ doc] $ marker , UnitsDisplayNames <'static >, # [cfg (feature = "datagen")] attributes_domain = "units") ;) * } ; }
};
}
