use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Level {
    /// Converts a level to a lower-case string.
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Allow => "allow",
            Level::Expect => "expect",
            Level::Warn => "warn",
            Level::ForceWarn => "force-warn",
            Level::Deny => "deny",
            Level::Forbid => "forbid",
        }
    }
    /// Converts a lower-case string to a level. This will never construct the expect
    /// level as that would require a [`LintExpectationId`].
    pub fn from_str(x: &str) -> Option<Self> {
        match x {
            "allow" => Some(Level::Allow),
            "warn" => Some(Level::Warn),
            "deny" => Some(Level::Deny),
            "forbid" => Some(Level::Forbid),
            "expect" | _ => None,
        }
    }
    /// Converts an `Attribute` to a level.
    pub fn from_attr(attr: &impl AttributeExt) -> Option<(Self, Option<LintExpectationId>)> {
        attr.name()
            .and_then(|name| Self::from_symbol(name, || Some(attr.id())))
    }
    /// Converts a `Symbol` to a level.
    pub fn from_symbol(
        s: Symbol,
        id: impl FnOnce() -> Option<AttrId>,
    ) -> Option<(Self, Option<LintExpectationId>)> {
        match s {
            sym::allow => Some((Level::Allow, None)),
            sym::expect => {
                if let Some(attr_id) = id() {
                    Some((
                        Level::Expect,
                        Some(LintExpectationId::Unstable {
                            attr_id,
                            lint_index: None,
                        }),
                    ))
                } else {
                    None
                }
            }
            sym::warn => Some((Level::Warn, None)),
            sym::deny => Some((Level::Deny, None)),
            sym::forbid => Some((Level::Forbid, None)),
            _ => None,
        }
    }
    pub fn to_cmd_flag(self) -> &'static str {
        match self {
            Level::Warn => "-W",
            Level::Deny => "-D",
            Level::Forbid => "-F",
            Level::Allow => "-A",
            Level::ForceWarn => "--force-warn",
            Level::Expect => {
                unreachable!("the expect level does not have a commandline flag")
            }
        }
    }
    pub fn is_error(self) -> bool {
        match self {
            Level::Allow | Level::Expect | Level::Warn | Level::ForceWarn => false,
            Level::Deny | Level::Forbid => true,
        }
    }
}
