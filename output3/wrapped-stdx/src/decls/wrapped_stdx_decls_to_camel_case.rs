use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[must_use]
pub fn to_camel_case(ident: &str) -> String {
    ident
        .trim_matches('_')
        .split('_')
        .filter(|component| !component.is_empty())
        .map(|component| {
            let mut camel_cased_component = String::with_capacity(component.len());
            let mut new_word = true;
            let mut prev_is_lower_case = true;
            for c in component.chars() {
                if prev_is_lower_case && c.is_uppercase() {
                    new_word = true;
                }
                if new_word {
                    camel_cased_component.extend(c.to_uppercase());
                } else {
                    camel_cased_component.extend(c.to_lowercase());
                }
                prev_is_lower_case = c.is_lowercase();
                new_word = false;
            }
            camel_cased_component
        })
        .fold(
            (String::new(), None),
            |(mut acc, prev): (_, Option<String>), next| {
                let join = prev
                    .and_then(|prev| {
                        let f = next.chars().next()?;
                        let l = prev.chars().last()?;
                        Some(!char_has_case(l) && !char_has_case(f))
                    })
                    .unwrap_or(false);
                acc.push_str(if join { "_" } else { "" });
                acc.push_str(&next);
                (acc, Some(next))
            },
        )
        .0
}
