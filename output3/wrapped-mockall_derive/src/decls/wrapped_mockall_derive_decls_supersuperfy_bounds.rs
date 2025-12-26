use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn supersuperfy_bounds(bounds: &mut Punctuated<TypeParamBound, Token![+]>, levels: usize) {
    for bound in bounds.iter_mut() {
        if let TypeParamBound::Trait(tb) = bound {
            supersuperfy_path(&mut tb.path, levels);
        }
    }
}
