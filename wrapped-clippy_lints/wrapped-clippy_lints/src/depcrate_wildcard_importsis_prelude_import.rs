// Generated macro for is_prelude_import (function)
macro_rules! Depcrate_wildcard_importsis_prelude_import {
() => {
// Module: crate::wildcard_imports
// Provides: {"is_prelude_import"}
// Dependencies: {}
fn is_prelude_import (segments : & [PathSegment < '_ >]) -> bool { segments . iter () . any (| ps | ps . ident . as_str () . contains ("prelude")) }
};
}
