macro_rules! deps {
    () => {
        CallItem!();
        CallLocations!();
        CallHierarchyConfig!();
    };
}

macro_rules! outgoing_calls {
    () => {
        deps!();
        pub (crate) fn outgoing_calls (db : & RootDatabase , config : & CallHierarchyConfig < '_ > , FilePosition { file_id , offset } : FilePosition ,) -> Option < Vec < CallItem > > { let sema = Semantics :: new (db) ; let file = sema . parse_guess_edition (file_id) ; let file = file . syntax () ; let token = pick_best_token (file . token_at_offset (offset) , | kind | match kind { IDENT => 1 , _ => 0 , }) ? ; let mut calls = CallLocations :: default () ; sema . descend_into_macros_exact (token) . into_iter () . filter_map (| it | it . parent_ancestors () . nth (1) . and_then (ast :: Item :: cast)) . filter_map (| item | match item { ast :: Item :: Const (c) => c . body () . map (| it | it . syntax () . descendants ()) , ast :: Item :: Fn (f) => f . body () . map (| it | it . syntax () . descendants ()) , ast :: Item :: Static (s) => s . body () . map (| it | it . syntax () . descendants ()) , _ => None , }) . flatten () . filter_map (ast :: CallableExpr :: cast) . filter_map (| call_node | { let (nav_target , range) = match call_node { ast :: CallableExpr :: Call (call) => { let expr = call . expr () ? ; let callable = sema . type_of_expr (& expr) ? . original . as_callable (db) ? ; match callable . kind () { hir :: CallableKind :: Function (it) => { if config . exclude_tests && it . is_test (db) { return None ; } it . try_to_nav (& sema) } hir :: CallableKind :: TupleEnumVariant (it) => it . try_to_nav (& sema) , hir :: CallableKind :: TupleStruct (it) => it . try_to_nav (& sema) , _ => None , } . zip (Some (sema . original_range (expr . syntax ()))) } ast :: CallableExpr :: MethodCall (expr) => { let function = sema . resolve_method_call (& expr) ? ; if config . exclude_tests && function . is_test (db) { return None ; } function . try_to_nav (& sema) . zip (Some (sema . original_range (expr . name_ref () ? . syntax ()))) } } ? ; Some (nav_target . into_iter () . zip (iter :: repeat (range))) }) . flatten () . for_each (| (nav , range) | calls . add (nav , range . into_file_id (db))) ; Some (calls . into_items ()) }
    };
}

outgoing_calls!();