macro_rules! deps {
    () => {
        EarlyContext!();
        UnusedImportBracesDiag!();
    };
}

macro_rules! impl_858 {
    () => {
        deps!();
        impl UnusedImportBraces { fn check_use_tree (& self , cx : & EarlyContext < '_ > , use_tree : & ast :: UseTree , item : & ast :: Item) { if let ast :: UseTreeKind :: Nested { ref items , .. } = use_tree . kind { for (tree , _) in items { self . check_use_tree (cx , tree , item) ; } let [(tree , _)] = items . as_slice () else { return } ; let node_name = match tree . kind { ast :: UseTreeKind :: Simple (rename) => { let orig_ident = tree . prefix . segments . last () . unwrap () . ident ; if orig_ident . name == kw :: SelfLower { return ; } rename . unwrap_or (orig_ident) . name } ast :: UseTreeKind :: Glob => sym :: asterisk , ast :: UseTreeKind :: Nested { .. } => return , } ; cx . emit_span_lint (UNUSED_IMPORT_BRACES , item . span , UnusedImportBracesDiag { node : node_name } ,) ; } } }
    };
}

impl_858!();