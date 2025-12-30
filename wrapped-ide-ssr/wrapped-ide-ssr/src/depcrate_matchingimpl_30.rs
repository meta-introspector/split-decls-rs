// Generated macro for impl_30 (impl)
macro_rules! Depcrate_matchingimpl_30 {
() => {
// Module: crate::matching
// Provides: {"impl_30"}
// Dependencies: {}
impl Match { fn render_template_paths < 'db > (& mut self , template : & ResolvedPattern < 'db > , sema : & Semantics < 'db , ide_db :: RootDatabase > ,) -> Result < () , MatchFailed > { let module = sema . scope (& self . matched_node) . ok_or_else (| | match_error ! ("Matched node isn't in a module")) ? . module () ; for (path , resolved_path) in & template . resolved_paths { if let hir :: PathResolution :: Def (module_def) = resolved_path . resolution { let cfg = FindPathConfig { prefer_no_std : false , prefer_prelude : true , prefer_absolute : false , allow_unstable : true , } ; let mod_path = module . find_path (sema . db , module_def , cfg) . ok_or_else (| | { match_error ! ("Failed to render template path `{}` at match location") }) ? ; self . rendered_template_paths . insert (path . clone () , mod_path) ; } } Ok (()) } }
};
}
