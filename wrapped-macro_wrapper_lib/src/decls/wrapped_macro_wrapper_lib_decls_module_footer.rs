use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[proc_macro]
pub fn module_footer(_input: ProcMacroTokenStream) -> ProcMacroTokenStream {
    quote! {}.into()
}
