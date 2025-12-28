macro_rules! deps {
    () => {
        ResolvedPattern!();
        Resolver!();
        Token!();
        SsrError!();
        UfcsCallInfo!();
        ResolvedPath!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < 'db > Resolver < '_ , 'db > { fn resolve_pattern_tree (& self , pattern : SyntaxNode) -> Result < ResolvedPattern < 'db > , SsrError > { use syntax :: ast :: AstNode ; use syntax :: { SyntaxElement , T } ; let mut resolved_paths = FxHashMap :: default () ; self . resolve (pattern . clone () , 0 , & mut resolved_paths) ? ; let ufcs_function_calls = resolved_paths . iter () . filter_map (| (path_node , resolved) | { if let Some (grandparent) = path_node . parent () . and_then (| parent | parent . parent ()) && let Some (call_expr) = ast :: CallExpr :: cast (grandparent . clone ()) && let hir :: PathResolution :: Def (hir :: ModuleDef :: Function (function)) = resolved . resolution && function . as_assoc_item (self . resolution_scope . scope . db) . is_some () { let qualifier_type = self . resolution_scope . qualifier_type (path_node) ; return Some ((grandparent , UfcsCallInfo { call_expr , function , qualifier_type } ,)) ; } None }) . collect () ; let contains_self = pattern . descendants_with_tokens () . any (| node_or_token | match node_or_token { SyntaxElement :: Token (t) => t . kind () == T ! [self] , _ => false , }) ; Ok (ResolvedPattern { node : pattern , resolved_paths , placeholders_by_stand_in : self . placeholders_by_stand_in . clone () , ufcs_function_calls , contains_self , }) } fn resolve (& self , node : SyntaxNode , depth : u32 , resolved_paths : & mut FxHashMap < SyntaxNode , ResolvedPath > ,) -> Result < () , SsrError > { use syntax :: ast :: AstNode ; if let Some (path) = ast :: Path :: cast (node . clone ()) { if is_self (& path) { return Ok (()) ; } if ! path_contains_type_arguments (path . qualifier ()) && ! self . path_contains_placeholder (& path) { let resolution = self . resolution_scope . resolve_path (& path) . ok_or_else (| | error ! ("Failed to resolve path `{}`" , node . text ())) ? ; if self . ok_to_use_path_resolution (& resolution) { resolved_paths . insert (node , ResolvedPath { resolution , depth }) ; return Ok (()) ; } } } for node in node . children () { self . resolve (node , depth + 1 , resolved_paths) ? ; } Ok (()) } # [doc = " Returns whether `path` contains a placeholder, but ignores any placeholders within type"] # [doc = " arguments."] fn path_contains_placeholder (& self , path : & ast :: Path) -> bool { if let Some (segment) = path . segment () && let Some (name_ref) = segment . name_ref () && self . placeholders_by_stand_in . contains_key (name_ref . text () . as_str ()) { return true ; } if let Some (qualifier) = path . qualifier () { return self . path_contains_placeholder (& qualifier) ; } false } fn ok_to_use_path_resolution (& self , resolution : & hir :: PathResolution) -> bool { match resolution { hir :: PathResolution :: Def (hir :: ModuleDef :: Function (function)) if function . as_assoc_item (self . resolution_scope . scope . db) . is_some () => { if function . self_param (self . resolution_scope . scope . db) . is_some () { true } else { cov_mark :: hit ! (replace_associated_trait_default_function_call) ; false } } hir :: PathResolution :: Def (def @ (hir :: ModuleDef :: Const (_) | hir :: ModuleDef :: TypeAlias (_)) ,) if def . as_assoc_item (self . resolution_scope . scope . db) . is_some () => { cov_mark :: hit ! (replace_associated_trait_constant) ; false } _ => true , } } }
    };
}

impl_81!();