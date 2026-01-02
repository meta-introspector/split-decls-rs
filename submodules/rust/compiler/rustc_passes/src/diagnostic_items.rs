mkuse!{use rustc_hir :: diagnostic_items :: DiagnosticItems ;}
mkuse!{use rustc_hir :: { Attribute , CRATE_OWNER_ID , OwnerId } ;}
mkuse!{use rustc_middle :: query :: { LocalCrate , Providers } ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_span :: def_id :: { DefId , LOCAL_CRATE } ;}
mkuse!{use rustc_span :: { Symbol , sym } ;}
mkuse!{use crate :: errors :: DuplicateDiagnosticItemInCrate ;}

macro_rules! observe_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function observe_item in module {}", module_path!());
    };
}

mkfn!{
    observe_item_introspect!();
    fn observe_item < 'tcx > (tcx : TyCtxt < 'tcx > , diagnostic_items : & mut DiagnosticItems , owner : OwnerId) { let attrs = tcx . hir_attrs (owner . into ()) ; if let Some (name) = extract (attrs) { collect_item (tcx , diagnostic_items , name , owner . to_def_id ()) ; } }
}

macro_rules! collect_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_item in module {}", module_path!());
    };
}

mkfn!{
    collect_item_introspect!();
    fn collect_item (tcx : TyCtxt < '_ > , items : & mut DiagnosticItems , name : Symbol , item_def_id : DefId) { items . id_to_name . insert (item_def_id , name) ; if let Some (original_def_id) = items . name_to_id . insert (name , item_def_id) { if original_def_id != item_def_id { report_duplicate_item (tcx , name , original_def_id , item_def_id) ; } } }
}

macro_rules! report_duplicate_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function report_duplicate_item in module {}", module_path!());
    };
}

mkfn!{
    report_duplicate_item_introspect!();
    fn report_duplicate_item (tcx : TyCtxt < '_ > , name : Symbol , original_def_id : DefId , item_def_id : DefId ,) { let orig_span = tcx . hir_span_if_local (original_def_id) ; let duplicate_span = tcx . hir_span_if_local (item_def_id) ; tcx . dcx () . emit_err (DuplicateDiagnosticItemInCrate { duplicate_span , orig_span , crate_name : tcx . crate_name (item_def_id . krate) , orig_crate_name : tcx . crate_name (original_def_id . krate) , different_crates : (item_def_id . krate != original_def_id . krate) , name , }) ; }
}

macro_rules! extract_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function extract in module {}", module_path!());
    };
}

mkfn!{
    extract_introspect!();
    # [doc = " Extract the first `rustc_diagnostic_item = \"$name\"` out of a list of attributes."] fn extract (attrs : & [Attribute]) -> Option < Symbol > { attrs . iter () . find_map (| attr | { if attr . has_name (sym :: rustc_diagnostic_item) { attr . value_str () } else { None } }) }
}

macro_rules! diagnostic_items_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function diagnostic_items in module {}", module_path!());
    };
}

mkfn!{
    diagnostic_items_introspect!();
    # [doc = " Traverse and collect the diagnostic items in the current"] fn diagnostic_items (tcx : TyCtxt < '_ > , _ : LocalCrate) -> DiagnosticItems { let mut diagnostic_items = DiagnosticItems :: default () ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . owners () . chain (std :: iter :: once (CRATE_OWNER_ID)) { observe_item (tcx , & mut diagnostic_items , id) ; } diagnostic_items }
}

macro_rules! all_diagnostic_items_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function all_diagnostic_items in module {}", module_path!());
    };
}

mkfn!{
    all_diagnostic_items_introspect!();
    # [doc = " Traverse and collect all the diagnostic items in all crates."] fn all_diagnostic_items (tcx : TyCtxt < '_ > , () : ()) -> DiagnosticItems { let mut items = DiagnosticItems :: default () ; for cnum in tcx . crates (()) . iter () . copied () . filter (| cnum | tcx . is_user_visible_dep (* cnum)) . chain (std :: iter :: once (LOCAL_CRATE)) { for (& name , & def_id) in & tcx . diagnostic_items (cnum) . name_to_id { collect_item (tcx , & mut items , name , def_id) ; } } items }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { providers . diagnostic_items = diagnostic_items ; providers . all_diagnostic_items = all_diagnostic_items ; }
}