use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// If the mocked signature contains any variadic parts, they need a pattern.
/// The pattern is required for the signature of the mock function, a Rust
/// function, even though it's not required in the signature of the foreign
/// function.
fn fix_elipses(sig: &mut Signature) {
    if let Some(variadic) = &mut sig.variadic {
        if variadic.pat.is_none() {
            let pat = PatIdent {
                attrs: vec![],
                by_ref: None,
                mutability: None,
                ident: format_ident!("_"),
                subpat: None,
            };
            let colon = Token![:](variadic.span());
            variadic.pat = Some((Box::new(Pat::Ident(pat)), colon));
        }
    }
}
