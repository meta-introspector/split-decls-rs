// Generated macro for PrintAttribute (trait)
macro_rules! Depcrate_attrs_pretty_printingPrintAttribute {
() => {
// Module: crate::attrs::pretty_printing
// Provides: {"PrintAttribute"}
// Dependencies: {}
# [doc = " This trait is used to print attributes in `rustc_hir_pretty`."] # [doc = ""] # [doc = " For structs and enums it can be derived using [`rustc_macros::PrintAttribute`]."] # [doc = " The output will look a lot like a `Debug` implementation, but fields of several types"] # [doc = " like [`Span`]s and empty tuples, are gracefully skipped so they don't clutter the"] # [doc = " representation much."] pub trait PrintAttribute { # [doc = " Whether or not this will render as something meaningful, or if it's skipped"] # [doc = " (which will force the containing struct to also skip printing a comma"] # [doc = " and the field name)."] fn should_render (& self) -> bool ; fn print_attribute (& self , p : & mut Printer) ; }
};
}
