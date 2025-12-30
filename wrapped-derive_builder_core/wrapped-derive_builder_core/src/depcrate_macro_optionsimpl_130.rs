// Generated macro for impl_130 (impl)
macro_rules! Depcrate_macro_optionsimpl_130 {
() => {
// Module: crate::macro_options
// Provides: {"impl_130"}
// Dependencies: {}
impl < 'a > Iterator for FieldIter < 'a > { type Item = FieldWithDefaults < 'a > ; fn next (& mut self) -> Option < Self :: Item > { self . 1 . next () . map (| field | FieldWithDefaults { parent : self . 0 , field , }) } }
};
}
