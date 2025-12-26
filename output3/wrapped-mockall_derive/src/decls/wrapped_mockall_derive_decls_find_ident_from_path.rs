use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn find_ident_from_path(path: &Path) -> (Ident, PathArguments) {
    if path.segments.len() != 1 {
        compile_error(
            path.span(),
            "mockall_derive only supports structs defined in the current module",
        );
        return (Ident::new("", path.span()), PathArguments::None);
    }
    let last_seg = path.segments.last().unwrap();
    (last_seg.ident.clone(), last_seg.arguments.clone())
}
