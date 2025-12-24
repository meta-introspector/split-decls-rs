use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl DemangleOptions {
    /// Construct a new `DemangleOptions` with the default values.
    pub fn new() -> Self {
        Default::default()
    }
    /// Do not display function arguments.
    pub fn no_params(mut self) -> Self {
        self.no_params = true;
        self
    }
    /// Do not display the function return type.
    pub fn no_return_type(mut self) -> Self {
        self.no_return_type = true;
        self
    }
    /// Hide type annotations in template value parameters.
    /// These are not needed to distinguish template instances
    /// so this can make it easier to match user-provided
    /// template instance names.
    pub fn hide_expression_literal_types(mut self) -> Self {
        self.hide_expression_literal_types = true;
        self
    }
    /// Set the limit on recursion depth during the demangling phase. A low
    /// limit will cause valid symbols to be rejected, but a high limit may
    /// allow pathological symbols to overflow the stack during demangling.
    /// The default value is 128.
    pub fn recursion_limit(mut self, limit: u32) -> Self {
        self.recursion_limit = Some(
            NonZeroU32::new(limit).expect("Recursion limit must be > 0"),
        );
        self
    }
}
