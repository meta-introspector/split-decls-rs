use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn lifetimes_to_generic_params(
    lv: &Punctuated<LifetimeParam, Token![,]>,
) -> Punctuated<GenericParam, Token![,]> {
    lv.iter()
        .map(|lt| GenericParam::Lifetime(lt.clone()))
        .collect()
}
