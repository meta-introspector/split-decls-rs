use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Implements one or more COM interfaces.
///
/// # Example
/// ```rust,no_run
/// use windows_core::*;
///
/// #[interface("094d70d6-5202-44b8-abb8-43860da5aca2")]
/// unsafe trait IValue: IUnknown {
///     fn GetValue(&self, value: *mut i32) -> HRESULT;
/// }
///
/// #[implement(IValue)]
/// struct Value(i32);
///
/// impl IValue_Impl for Value_Impl {
///     unsafe fn GetValue(&self, value: *mut i32) -> HRESULT {
///         *value = self.0;
///         HRESULT(0)
///     }
/// }
///
/// let object: IValue = Value(123).into();
/// // Call interface methods...
/// ```
#[proc_macro_attribute]
pub fn implement(
    attributes: proc_macro::TokenStream,
    type_tokens: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    implement_core(attributes.into(), type_tokens.into()).into()
}
