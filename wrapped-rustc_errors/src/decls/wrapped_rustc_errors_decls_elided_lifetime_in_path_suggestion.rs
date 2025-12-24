use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub fn elided_lifetime_in_path_suggestion(
    source_map: &SourceMap,
    n: usize,
    path_span: Span,
    incl_angl_brckt: bool,
    insertion_span: Span,
) -> ElidedLifetimeInPathSubdiag {
    let expected = ExpectedLifetimeParameter {
        span: path_span,
        count: n,
    };
    let indicate = source_map
        .is_span_accessible(insertion_span)
        .then(|| {
            let anon_lts = vec!["'_"; n].join(", ");
            let suggestion = if incl_angl_brckt {
                format!("<{anon_lts}>")
            } else {
                format!("{anon_lts}, ")
            };
            IndicateAnonymousLifetime {
                span: insertion_span.shrink_to_hi(),
                count: n,
                suggestion,
            }
        });
    ElidedLifetimeInPathSubdiag {
        expected,
        indicate,
    }
}
