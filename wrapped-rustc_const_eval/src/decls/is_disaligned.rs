macro_rules! deps {
    () => {
        Place!();
    };
}

macro_rules! is_disaligned {
    () => {
        deps!();
        # [doc = " Returns `true` if this place is allowed to be less aligned"] # [doc = " than its containing struct (because it is within a packed"] # [doc = " struct)."] pub fn is_disaligned < 'tcx , L > (tcx : TyCtxt < 'tcx > , local_decls : & L , typing_env : ty :: TypingEnv < 'tcx > , place : Place < 'tcx > ,) -> bool where L : HasLocalDecls < 'tcx > , { debug ! ("is_disaligned({:?})" , place) ; let Some (pack) = is_within_packed (tcx , local_decls , place) else { debug ! ("is_disaligned({:?}) - not within packed" , place) ; return false ; } ; let ty = place . ty (local_decls , tcx) . ty ; let unsized_tail = | | tcx . struct_tail_for_codegen (ty , typing_env) ; match tcx . layout_of (typing_env . as_query_input (ty)) { Ok (layout) if layout . align . abi <= pack && (layout . is_sized () || matches ! (unsized_tail () . kind () , ty :: Slice (..) | ty :: Str)) => { debug ! ("is_disaligned({:?}) - align = {}, packed = {}; not disaligned" , place , layout . align . abi . bytes () , pack . bytes ()) ; false } _ => { debug ! ("is_disaligned({:?}) - true" , place) ; true } } }
    };
}

is_disaligned!();