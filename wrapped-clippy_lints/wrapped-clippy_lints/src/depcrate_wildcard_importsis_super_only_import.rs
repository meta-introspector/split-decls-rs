// Generated macro for is_super_only_import (function)
macro_rules! Depcrate_wildcard_importsis_super_only_import {
() => {
// Module: crate::wildcard_imports
// Provides: {"is_super_only_import"}
// Dependencies: {}
fn is_super_only_import (segments : & [PathSegment < '_ >]) -> bool { segments . len () == 1 && segments [0] . ident . name == kw :: Super }
};
}
