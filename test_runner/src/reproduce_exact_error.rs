use syn::parse_file;

fn main() {
    // This is the exact problematic code from translation.rs
    let problematic_code = r#"
use std::borrow::Cow;
use std::env;
use std::error::Report;
use std::sync::Arc;

pub use rustc_error_messages::{FluentArgs, LazyFallbackBundle};
use tracing::{debug, trace};

use crate::error::{TranslateError, TranslateErrorKind};
use crate::snippet::Style;
use crate::{DiagArg, DiagMessage, FluentBundle};

impl Translator {
    pub fn translate_message(
        &self,
        message: &DiagMessage,
        args: &FluentArgs<'_>,
    ) -> Result<Cow<'_, str>, TranslateError> {
        trace!(?message, ?args);
        let fallback = || self.fallback_fluent_bundle.format_diagnostic(message, args);

        // This is the problematic try block that syn can't parse
        try {
            match self.fluent_bundle.as_ref().map(|b| translate_with_bundle(b)) {
                Some(Ok(translated)) => translated,
                Some(Err(error)) => {
                    debug!(?message, ?error, "failed to translate message");
                    fallback()
                }
                None => fallback(),
            }
        }
    }
}
"#;

    println!("🧪 Testing problematic try block code:");
    match parse_file(problematic_code) {
        Ok(_) => println!("✅ Parses successfully!"),
        Err(e) => {
            println!("❌ Failed: {}", e);
            println!("\n📍 This demonstrates the exact parsing issue in translation.rs");
        }
    }
}
