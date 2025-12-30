// Generated macro for handle_errors_and_replacements (function)
macro_rules! Depcrate_revision_spec_parse_delegatehandle_errors_and_replacements {
() => {
// Module: crate::revision::spec::parse::delegate
// Provides: {"handle_errors_and_replacements"}
// Dependencies: {}
fn handle_errors_and_replacements (destination : & mut Vec < Error > , objs : & mut HashSet < ObjectId > , errors : Vec < (ObjectId , Error) > , replacements : & mut Replacements ,) -> Option < () > { if errors . len () == objs . len () { destination . extend (errors . into_iter () . map (| (_ , err) | err)) ; None } else { for (obj , err) in errors { objs . remove (& obj) ; destination . push (err) ; } for (find , replace) in replacements { objs . remove (find) ; objs . insert (* replace) ; } Some (()) } }
};
}
