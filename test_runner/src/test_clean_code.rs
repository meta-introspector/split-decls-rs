use syn::parse_file;

fn main() {
    let test_code = r#"
use std::borrow::Cow;
use std::env;
use std::error::Report;
use std::sync::Arc;

pub use rustc_error_messages::{FluentArgs, LazyFallbackBundle};
use tracing::{debug, trace};

use crate::error::{TranslateError, TranslateErrorKind};
use crate::snippet::Style;
use crate::{DiagArg, DiagMessage, FluentBundle};

pub fn to_fluent_args<'iter>(iter: impl Iterator<Item = DiagArg<'iter>>) -> FluentArgs<'static> {
    let mut args = if let Some(size) = iter.size_hint().1 {
        FluentArgs::with_capacity(size)
    } else {
        FluentArgs::new()
    };

    for (k, v) in iter {
        args.set(k.clone(), v.clone());
    }

    args
}

#[derive(Clone)]
pub struct Translator {
    pub fluent_bundle: Option<Arc<FluentBundle>>,
    pub fallback_fluent_bundle: LazyFallbackBundle,
}

impl Translator {
    pub fn with_fallback_bundle(
        resources: Vec<&'static str>,
        with_directionality_markers: bool,
    ) -> Translator {
        Translator {
            fluent_bundle: None,
            fallback_fluent_bundle: crate::fallback_fluent_bundle(
                resources,
                with_directionality_markers,
            ),
        }
    }
}
"#;

    println!("🧪 Testing clean code without doc comments:");
    match parse_file(test_code) {
        Ok(_) => println!("✅ Parses successfully!"),
        Err(e) => println!("❌ Failed: {}", e),
    }
}
