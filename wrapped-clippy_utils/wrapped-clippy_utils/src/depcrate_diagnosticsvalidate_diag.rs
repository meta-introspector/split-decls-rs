// Generated macro for validate_diag (function)
macro_rules! Depcrate_diagnosticsvalidate_diag {
() => {
// Module: crate::diagnostics
// Provides: {"validate_diag"}
// Dependencies: {}
# [doc = " Makes sure that a diagnostic is well formed."] # [doc = ""] # [doc = " rustc debug asserts a few properties about spans,"] # [doc = " but the clippy repo uses a distributed rustc build with debug assertions disabled,"] # [doc = " so this has historically led to problems during subtree syncs where those debug assertions"] # [doc = " only started triggered there."] # [doc = ""] # [doc = " This function makes sure we also validate them in debug clippy builds."] # [cfg (debug_assertions)] fn validate_diag (diag : & Diag < '_ , impl EmissionGuarantee >) { let suggestions = match & diag . suggestions { Suggestions :: Enabled (suggs) => & * * suggs , Suggestions :: Sealed (suggs) => & * * suggs , Suggestions :: Disabled => return , } ; for substitution in suggestions . iter () . flat_map (| s | & s . substitutions) { assert_eq ! (substitution . parts . iter () . find (| SubstitutionPart { snippet , span } | snippet . is_empty () && span . is_empty ()) , None , "span must not be empty and have no suggestion") ; assert_eq ! (substitution . parts . array_windows () . find (| [a , b] | a . span . overlaps (b . span)) , None , "suggestion must not have overlapping parts") ; } }
};
}
