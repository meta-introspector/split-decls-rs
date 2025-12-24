use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn expander_to_proc_macro(
    expander: proc_macro_api::ProcMacro,
    ignored_macros: &[Box<str>],
) -> ProcMacro {
    let name = expander.name();
    let kind = match expander.kind() {
        proc_macro_api::ProcMacroKind::CustomDerive => ProcMacroKind::CustomDerive,
        proc_macro_api::ProcMacroKind::Bang => ProcMacroKind::Bang,
        proc_macro_api::ProcMacroKind::Attr => ProcMacroKind::Attr,
    };
    let disabled = ignored_macros.iter().any(|replace| **replace == *name);
    ProcMacro {
        name: intern::Symbol::intern(name),
        kind,
        expander: sync::Arc::new(Expander(expander)),
        disabled,
    }
}
