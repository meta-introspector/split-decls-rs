// Generated macro for is_name_ref_in_import (function)
macro_rules! Depcrate_searchis_name_ref_in_import {
() => {
// Module: crate::search
// Provides: {"is_name_ref_in_import"}
// Dependencies: {}
fn is_name_ref_in_import (name_ref : & ast :: NameRef) -> bool { name_ref . syntax () . parent () . and_then (ast :: PathSegment :: cast) . and_then (| it | it . parent_path () . top_path () . syntax () . parent ()) . is_some_and (| it | it . kind () == SyntaxKind :: USE_TREE) }
};
}
