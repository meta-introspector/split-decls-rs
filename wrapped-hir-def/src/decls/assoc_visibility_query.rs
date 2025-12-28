macro_rules! deps {
    () => {
        AssocItemId!();
        DefDatabase!();
        Visibility!();
    };
}

macro_rules! assoc_visibility_query {
    () => {
        deps!();
        # [doc = " Resolve visibility of a type alias."] pub (crate) fn assoc_visibility_query (db : & dyn DefDatabase , def : AssocItemId) -> Visibility { match def { AssocItemId :: FunctionId (function_id) => { let loc = function_id . lookup (db) ; trait_item_visibility (db , loc . container) . unwrap_or_else (| | { let source = loc . source (db) ; visibility_from_ast (db , function_id , source . map (| src | src . visibility ())) }) } AssocItemId :: ConstId (const_id) => { let loc = const_id . lookup (db) ; trait_item_visibility (db , loc . container) . unwrap_or_else (| | { let source = loc . source (db) ; visibility_from_ast (db , const_id , source . map (| src | src . visibility ())) }) } AssocItemId :: TypeAliasId (type_alias_id) => { let loc = type_alias_id . lookup (db) ; trait_item_visibility (db , loc . container) . unwrap_or_else (| | { let source = loc . source (db) ; visibility_from_ast (db , type_alias_id , source . map (| src | src . visibility ())) }) } } }
    };
}

assoc_visibility_query!()