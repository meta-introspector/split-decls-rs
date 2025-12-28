macro_rules! deps {
    () => {
        LoweringContext!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a , 'hir > LoweringContext < 'a , 'hir > { fn new (tcx : TyCtxt < 'hir > , resolver : & 'a mut ResolverAstLowering) -> Self { let registered_tools = tcx . registered_tools (()) . iter () . map (| x | x . name) . collect () ; Self { tcx , resolver , arena : tcx . hir_arena , bodies : Vec :: new () , define_opaque : None , attrs : SortedMap :: default () , children : Vec :: default () , contract_ensures : None , current_hir_id_owner : hir :: CRATE_OWNER_ID , item_local_id_counter : hir :: ItemLocalId :: ZERO , ident_and_label_to_local_id : Default :: default () , # [cfg (debug_assertions)] node_id_to_local_id : Default :: default () , trait_map : Default :: default () , catch_scope : None , loop_scope : None , is_in_loop_condition : false , is_in_dyn_type : false , coroutine_kind : None , task_context : None , current_item : None , impl_trait_defs : Vec :: new () , impl_trait_bounds : Vec :: new () , allow_try_trait : [sym :: try_trait_v2 , sym :: yeet_desugar_details] . into () , allow_pattern_type : [sym :: pattern_types , sym :: pattern_type_range_trait] . into () , allow_gen_future : if tcx . features () . async_fn_track_caller () { [sym :: gen_future , sym :: closure_track_caller] . into () } else { [sym :: gen_future] . into () } , allow_for_await : [sym :: async_iterator] . into () , allow_async_fn_traits : [sym :: async_fn_traits] . into () , allow_async_iterator : [sym :: gen_future , sym :: async_iterator] . into () , attribute_parser : AttributeParser :: new (tcx . sess , tcx . features () , registered_tools , Late ,) , delayed_lints : Vec :: new () , } } pub (crate) fn dcx (& self) -> DiagCtxtHandle < 'hir > { self . tcx . dcx () } }
    };
}

impl_14!()