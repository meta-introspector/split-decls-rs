use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parsed interface guid attribute
///
/// ```rust,ignore
/// #[windows_interface::interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
///                              //^ parses this
/// unsafe trait IUIAnimationVariable: IUnknown {
///     fn GetValue(&self, value: *mut f64) -> HRESULT;
/// }
/// ```
struct Guid(Option<syn::LitStr>);
