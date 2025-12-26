use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Defines a COM interface to call or implement.
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
pub fn interface(
    attributes: proc_macro::TokenStream,
    original_type: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let guid = syn::parse_macro_input!(attributes as Guid);
    let interface = syn::parse_macro_input!(original_type as Interface);
    let tokens = match interface.gen_tokens(&guid) {
        Ok(t) => t,
        Err(e) => return e.to_compile_error().into(),
    };
    tokens.into()
}
