macro_rules! deps {
    () => {
        OpTy!();
        CompileTimeInterpCx!();
        Immediate!();
    };
}

macro_rules! op_to_const {
    () => {
        deps!();
        # [doc = " This function converts an interpreter value into a MIR constant."] # [doc = ""] # [doc = " The `for_diagnostics` flag turns the usual rules for returning `ConstValue::Scalar` into a"] # [doc = " best-effort attempt. This is not okay for use in const-eval sine it breaks invariants rustc"] # [doc = " relies on, but it is okay for diagnostics which will just give up gracefully when they"] # [doc = " encounter an `Indirect` they cannot handle."] # [instrument (skip (ecx) , level = "debug")] pub (super) fn op_to_const < 'tcx > (ecx : & CompileTimeInterpCx < 'tcx > , op : & OpTy < 'tcx > , for_diagnostics : bool ,) -> ConstValue { if op . layout . is_zst () { return ConstValue :: ZeroSized ; } let force_as_immediate = match op . layout . backend_repr { BackendRepr :: Scalar (abi :: Scalar :: Initialized { .. }) => true , _ => false , } ; let immediate = if force_as_immediate { match ecx . read_immediate (op) . report_err () { Ok (imm) => Right (imm) , Err (err) => { if for_diagnostics { op . as_mplace_or_imm () } else { panic ! ("normalization works on validated constants: {err:?}") } } } } else { op . as_mplace_or_imm () } ; debug ! (? immediate) ; match immediate { Left (ref mplace) => { let (prov , offset) = mplace . ptr () . into_pointer_or_addr () . unwrap () . prov_and_relative_offset () ; let alloc_id = prov . alloc_id () ; ConstValue :: Indirect { alloc_id , offset } } Right (imm) => match * imm { Immediate :: Scalar (x) => ConstValue :: Scalar (x) , Immediate :: ScalarPair (a , b) => { debug ! ("ScalarPair(a: {:?}, b: {:?})" , a , b) ; let pointee_ty = imm . layout . ty . builtin_deref (false) . unwrap () ; debug_assert ! (matches ! (ecx . tcx . struct_tail_for_codegen (pointee_ty , ecx . typing_env ()) . kind () , ty :: Str | ty :: Slice (..) ,) , "`ConstValue::Slice` is for slice-tailed types only, but got {}" , imm . layout . ty ,) ; let msg = "`op_to_const` on an immediate scalar pair must only be used on slice references to the beginning of an actual allocation" ; let ptr = a . to_pointer (ecx) . expect (msg) ; let (prov , offset) = ptr . into_pointer_or_addr () . expect (msg) . prov_and_relative_offset () ; let alloc_id = prov . alloc_id () ; assert ! (offset == abi :: Size :: ZERO , "{}" , msg) ; let meta = b . to_target_usize (ecx) . expect (msg) ; ConstValue :: Slice { alloc_id , meta } } Immediate :: Uninit => bug ! ("`Uninit` is not a valid value for {}" , op . layout . ty) , } , } }
    };
}

op_to_const!()