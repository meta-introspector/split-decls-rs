macro_rules! deps {
    () => {
        DocumentationLinks!();
    };
}

macro_rules! external_docs {
    () => {
        deps!();
        pub (crate) fn external_docs (db : & RootDatabase , FilePosition { file_id , offset } : FilePosition , target_dir : Option < & str > , sysroot : Option < & str > ,) -> Option < DocumentationLinks > { let sema = & Semantics :: new (db) ; let file = sema . parse_guess_edition (file_id) . syntax () . clone () ; let token = pick_best_token (file . token_at_offset (offset) , | kind | match kind { IDENT | INT_NUMBER | T ! [self] => 3 , T ! ['('] | T ! [')'] => 2 , kind if kind . is_trivia () => 0 , _ => 1 , }) ? ; let token = sema . descend_into_macros_single_exact (token) ; let node = token . parent () ? ; let definition = match_ast ! { match node { ast :: NameRef (name_ref) => match NameRefClass :: classify (sema , & name_ref) ? { NameRefClass :: Definition (def , _) => def , NameRefClass :: FieldShorthand { local_ref : _ , field_ref , adt_subst : _ } => { Definition :: Field (field_ref) } NameRefClass :: ExternCrateShorthand { decl , .. } => { Definition :: ExternCrateDecl (decl) } } , ast :: Name (name) => match NameClass :: classify (sema , & name) ? { NameClass :: Definition (it) | NameClass :: ConstReference (it) => it , NameClass :: PatFieldShorthand { local_def : _ , field_ref , adt_subst : _ } => Definition :: Field (field_ref) , } , _ => return None } } ; Some (get_doc_links (db , definition , target_dir , sysroot)) }
    };
}

external_docs!();