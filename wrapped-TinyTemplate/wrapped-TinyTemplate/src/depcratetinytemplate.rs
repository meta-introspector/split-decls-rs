// Generated macro for TinyTemplate (struct)
macro_rules! DepcrateTinyTemplate {
() => {
// Module: crate
// Provides: {"TinyTemplate"}
// Dependencies: {}
# [doc = " The TinyTemplate struct is the entry point for the TinyTemplate library. It contains the"] # [doc = " template and formatter registries and provides functions to render templates as well as to"] # [doc = " register templates and formatters."] pub struct TinyTemplate < 'template > { templates : HashMap < & 'template str , Template < 'template > > , formatters : HashMap < & 'template str , Box < ValueFormatter > > , default_formatter : & 'template ValueFormatter , }
};
}
