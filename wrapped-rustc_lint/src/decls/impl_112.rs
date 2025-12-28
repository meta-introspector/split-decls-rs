macro_rules! deps {
    () => {
        EarlyContext!();
        BuiltinSpecialModuleNameUsed!();
    };
}

macro_rules! impl_112 {
    () => {
        deps!();
        impl EarlyLintPass for SpecialModuleName { fn check_crate (& mut self , cx : & EarlyContext < '_ > , krate : & ast :: Crate) { for item in & krate . items { if let ast :: ItemKind :: Mod (_ , ident , ast :: ModKind :: Unloaded | ast :: ModKind :: Loaded (_ , ast :: Inline :: No { .. } , _) ,) = item . kind { if item . attrs . iter () . any (| a | a . has_name (sym :: path)) { continue ; } match ident . name . as_str () { "lib" => cx . emit_span_lint (SPECIAL_MODULE_NAME , item . span , BuiltinSpecialModuleNameUsed :: Lib ,) , "main" => cx . emit_span_lint (SPECIAL_MODULE_NAME , item . span , BuiltinSpecialModuleNameUsed :: Main ,) , _ => continue , } } } } }
    };
}

impl_112!();