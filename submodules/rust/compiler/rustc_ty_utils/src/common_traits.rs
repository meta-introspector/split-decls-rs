mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_infer :: infer :: TyCtxtInferExt ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: { self , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use rustc_trait_selection :: traits ;}

macro_rules! is_copy_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_copy_raw in module {}", module_path!());
    };
}

mkfn!{
    is_copy_raw_introspect!();
    fn is_copy_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Copy) }
}

macro_rules! is_use_cloned_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_use_cloned_raw in module {}", module_path!());
    };
}

mkfn!{
    is_use_cloned_raw_introspect!();
    fn is_use_cloned_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { is_item_raw (tcx , query , LangItem :: UseCloned) }
}

macro_rules! is_sized_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_sized_raw in module {}", module_path!());
    };
}

mkfn!{
    is_sized_raw_introspect!();
    fn is_sized_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Sized) }
}

macro_rules! is_freeze_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_freeze_raw in module {}", module_path!());
    };
}

mkfn!{
    is_freeze_raw_introspect!();
    fn is_freeze_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Freeze) }
}

macro_rules! is_unpin_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_unpin_raw in module {}", module_path!());
    };
}

mkfn!{
    is_unpin_raw_introspect!();
    fn is_unpin_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > >) -> bool { is_item_raw (tcx , query , LangItem :: Unpin) }
}

macro_rules! is_async_drop_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_async_drop_raw in module {}", module_path!());
    };
}

mkfn!{
    is_async_drop_raw_introspect!();
    fn is_async_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { is_item_raw (tcx , query , LangItem :: AsyncDrop) }
}

macro_rules! is_item_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_item_raw in module {}", module_path!());
    };
}

mkfn!{
    is_item_raw_introspect!();
    fn is_item_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > , item : LangItem ,) -> bool { let (infcx , param_env) = tcx . infer_ctxt () . build_with_typing_env (query . typing_env) ; let trait_def_id = tcx . require_lang_item (item , DUMMY_SP) ; traits :: type_known_to_meet_bound_modulo_regions (& infcx , param_env , query . value , trait_def_id) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { is_copy_raw , is_use_cloned_raw , is_sized_raw , is_freeze_raw , is_unpin_raw , is_async_drop_raw , .. * providers } ; }
}