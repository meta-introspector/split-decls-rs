// Generated macro for impl_54 (impl)
macro_rules! Depcrate_confimpl_54 {
() => {
// Module: crate::conf
// Provides: {"impl_54"}
// Dependencies: {}
impl serde :: de :: Error for FieldError { fn custom < T : Display > (msg : T) -> Self { Self { error : msg . to_string () , suggestion : None , } } fn unknown_field (field : & str , expected : & 'static [& 'static str]) -> Self { use fmt :: Write ; let metadata = get_configuration_metadata () ; let deprecated = metadata . iter () . filter_map (| conf | { if conf . deprecation_reason . is_some () { Some (conf . name . as_str ()) } else { None } }) . collect :: < Vec < _ > > () ; let mut expected = expected . iter () . copied () . filter (| name | ! deprecated . contains (name)) . collect :: < Vec < _ > > () ; expected . sort_unstable () ; let (rows , column_widths) = calculate_dimensions (& expected) ; let mut msg = format ! ("unknown field `{field}`, expected one of") ; for row in 0 .. rows { writeln ! (msg) . unwrap () ; for (column , column_width) in column_widths . iter () . copied () . enumerate () { let index = column * rows + row ; let field = expected . get (index) . copied () . unwrap_or_default () ; write ! (msg , "{:SEPARATOR_WIDTH$}{field:column_width$}" , " ") . unwrap () ; } } let suggestion = suggest_candidate (field , expected) . map (| suggestion | Suggestion { message : "perhaps you meant" , suggestion , }) ; Self { error : msg , suggestion } } }
};
}
