macro_rules! deps {
    () => {
        Runnable!();
        TestId!();
        RunnableKind!();
        NavigationTarget!();
        UpdateTest!();
    };
}

macro_rules! module_def_doctest {
    () => {
        deps!();
        fn module_def_doctest (sema : & Semantics < '_ , RootDatabase > , def : Definition) -> Option < Runnable > { let db = sema . db ; let attrs = match def { Definition :: Module (it) => it . attrs (db) , Definition :: Function (it) => it . attrs (db) , Definition :: Adt (it) => it . attrs (db) , Definition :: Variant (it) => it . attrs (db) , Definition :: Const (it) => it . attrs (db) , Definition :: Static (it) => it . attrs (db) , Definition :: Trait (it) => it . attrs (db) , Definition :: TypeAlias (it) => it . attrs (db) , Definition :: Macro (it) => it . attrs (db) , Definition :: SelfType (it) => it . attrs (db) , _ => return None , } ; let krate = def . krate (db) ; let edition = krate . map (| it | it . edition (db)) . unwrap_or (Edition :: CURRENT) ; let display_target = krate . unwrap_or_else (| | (* db . all_crates () . last () . expect ("no crate graph present")) . into ()) . to_display_target (db) ; if ! has_runnable_doc_test (& attrs) { return None ; } let def_name = def . name (db) ? ; let path = (| | { let mut path = String :: new () ; def . canonical_module_path (db) ? . flat_map (| it | it . name (db)) . for_each (| name | format_to ! (path , "{}::" , name . display (db , edition))) ; if let Some (assoc_item) = def . as_assoc_item (db) && let Some (ty) = assoc_item . implementing_ty (db) && let Some (adt) = ty . as_adt () { let name = adt . name (db) ; let mut ty_args = ty . generic_parameters (db , display_target) . peekable () ; format_to ! (path , "{}" , name . display (db , edition)) ; if ty_args . peek () . is_some () { hir :: attach_db (db , | | { format_to ! (path , "<{}>" , ty_args . format_with ("," , | ty , cb | cb (& ty))) ; }) ; } format_to ! (path , "::{}" , def_name . display (db , edition)) ; path . retain (| c | c != ' ') ; return Some (path) ; } format_to ! (path , "{}" , def_name . display (db , edition)) ; Some (path) }) () ; let test_id = path . map_or_else (| | TestId :: Name (def_name . display_no_db (edition) . to_smolstr ()) , TestId :: Path) ; let mut nav = match def { Definition :: Module (def) => NavigationTarget :: from_module_to_decl (db , def) , def => def . try_to_nav (sema) ? , } . call_site () ; nav . focus_range = None ; nav . description = None ; nav . docs = None ; nav . kind = None ; let res = Runnable { use_name_in_title : false , nav , kind : RunnableKind :: DocTest { test_id } , cfg : attrs . cfg () , update_test : UpdateTest :: default () , } ; Some (res) }
    };
}

module_def_doctest!();