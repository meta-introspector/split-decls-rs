// Generated macro for impl_marker_length_constructors (macro)
macro_rules! Depcrate_fieldsetsimpl_marker_length_constructors {
() => {
// Module: crate::fieldsets
// Provides: {"impl_marker_length_constructors"}
// Dependencies: {}
macro_rules ! impl_marker_length_constructors { ($ type : ident , $ (alignment : $ alignment_yes : ident ,) ? $ (year_style : $ yearstyle_yes : ident ,) ? $ (time_precision : $ timeprecision_yes : ident ,) ?) => { impl $ type { # [doc = concat ! ("Creates a " , stringify ! ($ type) , " skeleton with the given formatting length.")] pub const fn for_length (length : Length) -> Self { Self { length , $ (alignment : yes_to ! (None , $ alignment_yes) ,) ? $ (year_style : yes_to ! (None , $ yearstyle_yes) ,) ? $ (time_precision : yes_to ! (None , $ timeprecision_yes) ,) ? } } # [doc = concat ! ("Creates a " , stringify ! ($ type) , " skeleton with a long length.")] pub const fn long () -> Self { Self :: for_length (Length :: Long) } # [doc = concat ! ("Creates a " , stringify ! ($ type) , " skeleton with a medium length.")] pub const fn medium () -> Self { Self :: for_length (Length :: Medium) } # [doc = concat ! ("Creates a " , stringify ! ($ type) , " skeleton with a short length.")] pub const fn short () -> Self { Self :: for_length (Length :: Short) } } } ; }
};
}
