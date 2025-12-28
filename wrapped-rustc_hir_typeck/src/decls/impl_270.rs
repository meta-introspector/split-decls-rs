macro_rules! deps {
    () => {
        Pick!();
    };
}

macro_rules! impl_270 {
    () => {
        deps!();
        impl < 'tcx > Pick < 'tcx > { # [doc = " In case there were unstable name collisions, emit them as a lint."] # [doc = " Checks whether two picks do not refer to the same trait item for the same `Self` type."] # [doc = " Only useful for comparisons of picks in order to improve diagnostics."] # [doc = " Do not use for type checking."] pub (crate) fn differs_from (& self , other : & Self) -> bool { let Self { item : AssocItem { def_id , kind : _ , container : _ } , kind : _ , import_ids : _ , autoderefs : _ , autoref_or_ptr_adjustment : _ , self_ty , unstable_candidates : _ , receiver_steps : _ , shadowed_candidates : _ , } = * self ; self_ty != other . self_ty || def_id != other . item . def_id } # [doc = " In case there were unstable name collisions, emit them as a lint."] pub (crate) fn maybe_emit_unstable_name_collision_hint (& self , tcx : TyCtxt < 'tcx > , span : Span , scope_expr_id : HirId ,) { if self . unstable_candidates . is_empty () { return ; } let def_kind = self . item . as_def_kind () ; tcx . node_span_lint (lint :: builtin :: UNSTABLE_NAME_COLLISIONS , scope_expr_id , span , | lint | { lint . primary_message (format ! ("{} {} with this name may be added to the standard library in the future" , tcx . def_kind_descr_article (def_kind , self . item . def_id) , tcx . def_kind_descr (def_kind , self . item . def_id) ,)) ; match (self . item . kind , self . item . container) { (ty :: AssocKind :: Fn { .. } , _) => { lint . help (format ! ("call with fully qualified syntax `{}(...)` to keep using the current \
                             method" , tcx . def_path_str (self . item . def_id) ,)) ; } (ty :: AssocKind :: Const { name } , ty :: AssocContainer :: Trait) => { let def_id = self . item . container_id (tcx) ; lint . span_suggestion (span , "use the fully qualified path to the associated const" , format ! ("<{} as {}>::{}" , self . self_ty , tcx . def_path_str (def_id) , name) , Applicability :: MachineApplicable ,) ; } _ => { } } tcx . disabled_nightly_features (lint , self . unstable_candidates . iter () . map (| (candidate , feature) | { (format ! (" `{}`" , tcx . def_path_str (candidate . item . def_id)) , * feature) }) ,) ; }) ; } }
    };
}

impl_270!();