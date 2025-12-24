use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn build_lints_map(
    lints: &'static [Lint],
    lint_group: &'static [LintGroup],
    prefix: &'static str,
) -> FxHashMap<&'static str, BuiltLint> {
    let mut map_with_prefixes: FxHashMap<_, _> = lints
        .iter()
        .map(|lint| (
            lint.label,
            BuiltLint {
                lint,
                groups: vec![lint.label, "__RA_EVERY_LINT"],
            },
        ))
        .collect();
    for g in lint_group {
        let mut add_children = |label: &'static str| {
            for child in g.children {
                map_with_prefixes.get_mut(child).unwrap().groups.push(label);
            }
        };
        add_children(g.lint.label);
        if g.lint.label == "nonstandard_style" {
            add_children("bad_style");
        }
    }
    map_with_prefixes
        .into_iter()
        .map(|(k, v)| (k.strip_prefix(prefix).unwrap(), v))
        .collect()
}
