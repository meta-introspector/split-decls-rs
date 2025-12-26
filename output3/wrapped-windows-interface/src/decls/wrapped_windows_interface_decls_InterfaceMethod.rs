use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A parsed interface method
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
/// unsafe trait IUIAnimationVariable: IUnknown {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
///   //^ parses this
/// }
/// ```
struct InterfaceMethod {
    pub name: syn::Ident,
    pub visibility: syn::Visibility,
    pub args: Vec<InterfaceMethodArg>,
    pub ret: syn::ReturnType,
    pub docs: Vec<syn::Attribute>,
}
