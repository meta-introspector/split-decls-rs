macro_rules! documentation_for_definition {
    () => {
        fn documentation_for_definition (sema : & Semantics < '_ , RootDatabase > , def : Definition , scope_node : & SyntaxNode ,) -> Option < Documentation > { let famous_defs = match & def { Definition :: BuiltinType (_) => Some (FamousDefs (sema , sema . scope (scope_node) ? . krate ())) , _ => None , } ; def . docs (sema . db , famous_defs . as_ref () , def . krate (sema . db) . unwrap_or_else (| | { (* sema . db . all_crates () . last () . expect ("no crate graph present")) . into () }) . to_display_target (sema . db) ,) }
    };
}

documentation_for_definition!();