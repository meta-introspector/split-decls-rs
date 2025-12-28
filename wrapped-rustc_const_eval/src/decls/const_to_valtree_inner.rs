macro_rules! deps {
    () => {
        MPlaceTy!();
        CompileTimeInterpCx!();
        Coroutine!();
    };
}

macro_rules! const_to_valtree_inner {
    () => {
        deps!();
        # [instrument (skip (ecx) , level = "debug")] fn const_to_valtree_inner < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , place : & MPlaceTy < 'tcx > , num_nodes : & mut usize ,) -> EvalToValTreeResult < 'tcx > { let tcx = * ecx . tcx ; let ty = place . layout . ty ; debug ! ("ty kind: {:?}" , ty . kind ()) ; if * num_nodes >= VALTREE_MAX_NODES { return Err (ValTreeCreationError :: NodesOverflow) ; } match ty . kind () { ty :: FnDef (..) => { * num_nodes += 1 ; Ok (ty :: ValTree :: zst (tcx)) } ty :: Bool | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Char => { let val = ecx . read_immediate (place) . report_err () ? ; let val = val . to_scalar_int () . unwrap () ; * num_nodes += 1 ; Ok (ty :: ValTree :: from_scalar_int (tcx , val)) } ty :: Pat (base , ..) => { let mut place = place . clone () ; place . layout = ecx . layout_of (* base) . unwrap () ; ensure_sufficient_stack (| | const_to_valtree_inner (ecx , & place , num_nodes)) } , ty :: RawPtr (_ , _) => { let val = ecx . read_immediate (place) . report_err () ? ; if matches ! (val . layout . backend_repr , BackendRepr :: ScalarPair (..)) { return Err (ValTreeCreationError :: NonSupportedType (ty)) ; } let val = val . to_scalar () ; let Ok (val) = val . try_to_scalar_int () else { return Err (ValTreeCreationError :: NonSupportedType (ty)) ; } ; Ok (ty :: ValTree :: from_scalar_int (tcx , val)) } ty :: FnPtr (..) => Err (ValTreeCreationError :: NonSupportedType (ty)) , ty :: Ref (_ , _ , _) => { let derefd_place = ecx . deref_pointer (place) . report_err () ? ; const_to_valtree_inner (ecx , & derefd_place , num_nodes) } ty :: Str | ty :: Slice (_) | ty :: Array (_ , _) => { slice_branches (ecx , place , num_nodes) } ty :: Dynamic (..) => Err (ValTreeCreationError :: NonSupportedType (ty)) , ty :: Tuple (elem_tys) => { branches (ecx , place , elem_tys . len () , None , num_nodes) } ty :: Adt (def , _) => { if def . is_union () { return Err (ValTreeCreationError :: NonSupportedType (ty)) ; } else if def . variants () . is_empty () { bug ! ("uninhabited types should have errored and never gotten converted to valtree") } let variant = ecx . read_discriminant (place) . report_err () ? ; branches (ecx , place , def . variant (variant) . fields . len () , def . is_enum () . then_some (variant) , num_nodes) } ty :: Never | ty :: Error (_) | ty :: Foreign (..) | ty :: Infer (ty :: FreshIntTy (_)) | ty :: Infer (ty :: FreshFloatTy (_)) | ty :: Alias (..) | ty :: Param (_) | ty :: Bound (..) | ty :: Placeholder (..) | ty :: Infer (_) | ty :: Closure (..) | ty :: CoroutineClosure (..) | ty :: Coroutine (..) | ty :: CoroutineWitness (..) | ty :: UnsafeBinder (_) => Err (ValTreeCreationError :: NonSupportedType (ty)) , } }
    };
}

const_to_valtree_inner!();