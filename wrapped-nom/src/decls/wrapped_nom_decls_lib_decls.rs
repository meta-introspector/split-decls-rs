use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod decls {
    use introspector_decl2_macros::decl_module;
    use quote::quote;
    use introspector_decl_core;
    use introspector_decl_common;
    use proc_macro2;
    use syn;
    include!("decls/_decl_module_invocation.rs");
}
