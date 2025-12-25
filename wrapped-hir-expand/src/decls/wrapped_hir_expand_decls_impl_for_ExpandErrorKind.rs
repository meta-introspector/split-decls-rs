use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl ExpandErrorKind {
    pub fn render_to_string(&self, db: &dyn ExpandDatabase) -> RenderedExpandError {
        match self {
            ExpandErrorKind::ProcMacroAttrExpansionDisabled => RenderedExpandError {
                message: "procedural attribute macro expansion is disabled".to_owned(),
                error: false,
                kind: RenderedExpandError::ATTR_EXP_DISABLED,
            },
            ExpandErrorKind::MacroDisabled => RenderedExpandError {
                message: "proc-macro is explicitly disabled".to_owned(),
                error: false,
                kind: RenderedExpandError::DISABLED,
            },
            &ExpandErrorKind::MissingProcMacroExpander(def_crate) => {
                match db
                    .proc_macros_for_crate(def_crate)
                    .as_ref()
                    .and_then(|it| it.get_error())
                {
                    Some(e) => {
                        RenderedExpandError {
                            message: e.to_string(),
                            error: e.is_hard_error(),
                            kind: RenderedExpandError::GENERAL_KIND,
                        }
                    }
                    None => {
                        RenderedExpandError {
                            message: format!(
                                "internal error: proc-macro map is missing error entry for crate {def_crate:?}"
                            ),
                            error: true,
                            kind: RenderedExpandError::GENERAL_KIND,
                        }
                    }
                }
            }
            ExpandErrorKind::MacroDefinition => RenderedExpandError {
                message: "macro definition has parse errors".to_owned(),
                error: true,
                kind: RenderedExpandError::GENERAL_KIND,
            },
            ExpandErrorKind::Mbe(e) => RenderedExpandError {
                message: e.to_string(),
                error: true,
                kind: RenderedExpandError::GENERAL_KIND,
            },
            ExpandErrorKind::RecursionOverflow => RenderedExpandError {
                message: "overflow expanding the original macro".to_owned(),
                error: true,
                kind: RenderedExpandError::GENERAL_KIND,
            },
            ExpandErrorKind::Other(e) => RenderedExpandError {
                message: (**e).to_owned(),
                error: true,
                kind: RenderedExpandError::GENERAL_KIND,
            },
            ExpandErrorKind::ProcMacroPanic(e) => RenderedExpandError {
                message: format!("proc-macro panicked: {e}"),
                error: true,
                kind: RenderedExpandError::GENERAL_KIND,
            },
        }
    }
}
