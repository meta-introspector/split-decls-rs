use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn make_trait_method(sig: syn::Signature) -> TraitItemFn {
    TraitItemFn {
        attrs: vec![],
        sig: sig.clone(),
        semi_token: Some(syn::Token![;](sig.span())),
        default: None,
    }
}
