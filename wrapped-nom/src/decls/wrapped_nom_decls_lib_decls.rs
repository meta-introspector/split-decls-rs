use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod decls {
    use introspector_decl2_macros::decl_module;
    use introspector_decl_common;
    use introspector_decl_core;
    use proc_macro2;
    use quote::quote;
    use syn;
    include!("decls/_decl_module_invocation.rs");
}
