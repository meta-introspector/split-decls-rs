macro_rules! add_concrete_opaque_type {
    () => {
        # [doc = " Collect all defining uses of opaque types inside of this typeck root. This"] # [doc = " expects the hidden type to be mapped to the definition parameters of the opaque"] # [doc = " and errors if we end up with distinct hidden types."] fn add_concrete_opaque_type < 'tcx > (tcx : TyCtxt < 'tcx > , concrete_opaque_types : & mut ConcreteOpaqueTypes < 'tcx > , def_id : LocalDefId , hidden_ty : OpaqueHiddenType < 'tcx > ,) { if let Some (prev) = concrete_opaque_types . 0 . get_mut (& def_id) { if prev . ty != hidden_ty . ty { let guar = hidden_ty . ty . error_reported () . err () . unwrap_or_else (| | { let (Ok (e) | Err (e)) = prev . build_mismatch_error (& hidden_ty , tcx) . map (| d | d . emit ()) ; e }) ; prev . ty = Ty :: new_error (tcx , guar) ; } prev . span = prev . span . substitute_dummy (hidden_ty . span) ; } else { concrete_opaque_types . 0 . insert (def_id , hidden_ty) ; } }
    };
}

add_concrete_opaque_type!()