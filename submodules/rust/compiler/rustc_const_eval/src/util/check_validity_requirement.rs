mkuse!{use rustc_abi :: { BackendRepr , FieldsShape , Scalar , Variants } ;}
mkuse!{use rustc_middle :: ty :: layout :: { HasTyCtxt , LayoutCx , LayoutError , LayoutOf , TyAndLayout , ValidityRequirement , } ;}
mkuse!{use rustc_middle :: ty :: { PseudoCanonicalInput , ScalarInt , Ty , TyCtxt } ;}
mkuse!{use rustc_middle :: { bug , ty } ;}
mkuse!{use rustc_span :: DUMMY_SP ;}
mkuse!{use crate :: const_eval :: { CanAccessMutGlobal , CheckAlignment , CompileTimeMachine } ;}
mkuse!{use crate :: interpret :: { InterpCx , MemoryKind } ;}

macro_rules! check_validity_requirement_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_validity_requirement in module {}", module_path!());
    };
}

mkfn!{
    check_validity_requirement_introspect!();
    # [doc = " Determines if this type permits \"raw\" initialization by just transmuting some memory into an"] # [doc = " instance of `T`."] # [doc = ""] # [doc = " `init_kind` indicates if the memory is zero-initialized or left uninitialized. We assume"] # [doc = " uninitialized memory is mitigated by filling it with 0x01, which reduces the chance of causing"] # [doc = " LLVM UB."] # [doc = ""] # [doc = " By default we check whether that operation would cause *LLVM UB*, i.e., whether the LLVM IR we"] # [doc = " generate has UB or not. This is a mitigation strategy, which is why we are okay with accepting"] # [doc = " Rust UB as long as there is no risk of miscompilations. The `strict_init_checks` can be set to"] # [doc = " do a full check against Rust UB instead (in which case we will also ignore the 0x01-filling and"] # [doc = " to the full uninit check)."] pub fn check_validity_requirement < 'tcx > (tcx : TyCtxt < 'tcx > , kind : ValidityRequirement , input : PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> Result < bool , & 'tcx LayoutError < 'tcx > > { let layout = tcx . layout_of (input) ? ; if kind == ValidityRequirement :: Inhabited { return Ok (! layout . is_uninhabited ()) ; } let layout_cx = LayoutCx :: new (tcx , input . typing_env) ; if kind == ValidityRequirement :: Uninit || tcx . sess . opts . unstable_opts . strict_init_checks { Ok (check_validity_requirement_strict (layout , & layout_cx , kind)) } else { check_validity_requirement_lax (layout , & layout_cx , kind) } }
}

macro_rules! check_validity_requirement_strict_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_validity_requirement_strict in module {}", module_path!());
    };
}

mkfn!{
    check_validity_requirement_strict_introspect!();
    # [doc = " Implements the 'strict' version of the [`check_validity_requirement`] checks; see that function"] # [doc = " for details."] fn check_validity_requirement_strict < 'tcx > (ty : TyAndLayout < 'tcx > , cx : & LayoutCx < 'tcx > , kind : ValidityRequirement ,) -> bool { let machine = CompileTimeMachine :: new (CanAccessMutGlobal :: No , CheckAlignment :: Error) ; let mut cx = InterpCx :: new (cx . tcx () , DUMMY_SP , cx . typing_env , machine) ; let allocated = cx . allocate (ty , MemoryKind :: Stack) . expect ("OOM: failed to allocate for uninit check") ; if kind == ValidityRequirement :: Zero { cx . write_bytes_ptr (allocated . ptr () , std :: iter :: repeat (0_u8) . take (ty . layout . size () . bytes_usize ()) ,) . expect ("failed to write bytes for zero valid check") ; } cx . validate_operand (& allocated . into () , false , false ,) . discard_err () . is_some () }
}

macro_rules! check_validity_requirement_lax_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_validity_requirement_lax in module {}", module_path!());
    };
}

mkfn!{
    check_validity_requirement_lax_introspect!();
    # [doc = " Implements the 'lax' (default) version of the [`check_validity_requirement`] checks; see that"] # [doc = " function for details."] fn check_validity_requirement_lax < 'tcx > (this : TyAndLayout < 'tcx > , cx : & LayoutCx < 'tcx > , init_kind : ValidityRequirement ,) -> Result < bool , & 'tcx LayoutError < 'tcx > > { let scalar_allows_raw_init = move | s : Scalar | -> bool { match init_kind { ValidityRequirement :: Inhabited => { bug ! ("ValidityRequirement::Inhabited should have been handled above") } ValidityRequirement :: Zero => { s . valid_range (cx) . contains (0) } ValidityRequirement :: UninitMitigated0x01Fill => { let mut val : u128 = 0x01 ; for _ in 1 .. s . size (cx) . bytes () { val = (val << 8) | 0x01 ; } s . valid_range (cx) . contains (val) } ValidityRequirement :: Uninit => { bug ! ("ValidityRequirement::Uninit should have been handled above") } } } ; let valid = ! this . is_uninhabited () && match this . backend_repr { BackendRepr :: Scalar (s) => scalar_allows_raw_init (s) , BackendRepr :: ScalarPair (s1 , s2) => { scalar_allows_raw_init (s1) && scalar_allows_raw_init (s2) } BackendRepr :: SimdVector { element : s , count } => count == 0 || scalar_allows_raw_init (s) , BackendRepr :: Memory { .. } => true , } ; if ! valid { return Ok (false) ; } if let Some (pointee) = this . ty . builtin_deref (false) { let pointee = cx . layout_of (pointee) ? ; if pointee . align . abi . bytes () > 1 { return Ok (false) ; } if pointee . size . bytes () > 0 { return Ok (false) ; } } match & this . fields { FieldsShape :: Primitive | FieldsShape :: Union { .. } => { } FieldsShape :: Array { .. } => { } FieldsShape :: Arbitrary { offsets , .. } => { for idx in 0 .. offsets . len () { if ! check_validity_requirement_lax (this . field (cx , idx) , cx , init_kind) ? { return Ok (false) ; } } } } match & this . variants { Variants :: Empty => return Ok (false) , Variants :: Single { .. } => { } Variants :: Multiple { .. } => { } } Ok (true) }
}

macro_rules! validate_scalar_in_layout_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function validate_scalar_in_layout in module {}", module_path!());
    };
}

mkfn!{
    validate_scalar_in_layout_introspect!();
    pub (crate) fn validate_scalar_in_layout < 'tcx > (tcx : TyCtxt < 'tcx > , scalar : ScalarInt , ty : Ty < 'tcx > ,) -> bool { let machine = CompileTimeMachine :: new (CanAccessMutGlobal :: No , CheckAlignment :: Error) ; let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let mut cx = InterpCx :: new (tcx , DUMMY_SP , typing_env , machine) ; let Ok (layout) = cx . layout_of (ty) else { bug ! ("could not compute layout of {scalar:?}:{ty:?}") } ; let allocated = cx . allocate (layout , MemoryKind :: Stack) . expect ("OOM: failed to allocate for uninit check") ; cx . write_scalar (scalar , & allocated) . unwrap () ; cx . validate_operand (& allocated . into () , false , false ,) . discard_err () . is_some () }
}