// SRC: ../rust/compiler/rustc_expand/src/lib.rs
// tidy-alphabetical-start
#[allow(internal_features)]
#[allow(rustc::diagnostic_outside_of_impl)]
#[doc(rust_logo)]
#[feature(array_windows)]
#[feature(associated_type_defaults)]
#[feature(if_let_guard)]
#[feature(macro_metavar_expr)]
#[feature(proc_macro_diagnostic)]
#[feature(proc_macro_internals)]
#[feature(rustdoc_internals)]
#[feature(try_blocks)]
#[feature(yeet_expr)]
// tidy-alphabetical-end

// FIXME(Nilstrieb) Translate macro_rules diagnostics
#[allow(rustc::untranslatable_diagnostic)]

pub use mbe::macro_rules::{MacroRulesMacroExpander, compile_declarative_macro};
// FIXME(Nilstrieb) Translate proc_macro diagnostics
#[allow(rustc::untranslatable_diagnostic)]

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }