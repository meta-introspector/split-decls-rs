macro_rules! deps {
    () => {
        TypeKind!();
        BuilderMethods!();
    };
}

macro_rules! transmute_scalar {
    () => {
        deps!();
        # [doc = " Transmutes a single scalar value `imm` from `from_scalar` to `to_scalar`."] # [doc = ""] # [doc = " This is expected to be in *immediate* form, as seen in [`OperandValue::Immediate`]"] # [doc = " or [`OperandValue::Pair`] (so `i1` for bools, not `i8`, for example)."] # [doc = ""] # [doc = " ICEs if the passed-in `imm` is not a value of the expected type for"] # [doc = " `from_scalar`, such as if it's a vector or a pair."] pub (super) fn transmute_scalar < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , mut imm : Bx :: Value , from_scalar : abi :: Scalar , to_scalar : abi :: Scalar ,) -> Bx :: Value { assert_eq ! (from_scalar . size (bx . cx ()) , to_scalar . size (bx . cx ())) ; let imm_ty = bx . cx () . val_ty (imm) ; assert_ne ! (bx . cx () . type_kind (imm_ty) , TypeKind :: Vector , "Vector type {imm_ty:?} not allowed in transmute_scalar {from_scalar:?} -> {to_scalar:?}") ; if from_scalar == to_scalar { return imm ; } use abi :: Primitive :: * ; imm = bx . from_immediate (imm) ; let from_backend_ty = bx . cx () . type_from_scalar (from_scalar) ; debug_assert_eq ! (bx . cx () . val_ty (imm) , from_backend_ty) ; let to_backend_ty = bx . cx () . type_from_scalar (to_scalar) ; assume_scalar_range (bx , imm , from_scalar , from_backend_ty , Some (& to_scalar)) ; imm = match (from_scalar . primitive () , to_scalar . primitive ()) { (Int (..) | Float (_) , Int (..) | Float (_)) => bx . bitcast (imm , to_backend_ty) , (Pointer (..) , Pointer (..)) => bx . pointercast (imm , to_backend_ty) , (Int (..) , Pointer (..)) => bx . inttoptr (imm , to_backend_ty) , (Pointer (..) , Int (..)) => { bx . ptrtoint (imm , to_backend_ty) } (Float (_) , Pointer (..)) => { let int_imm = bx . bitcast (imm , bx . cx () . type_isize ()) ; bx . inttoptr (int_imm , to_backend_ty) } (Pointer (..) , Float (_)) => { let int_imm = bx . ptrtoint (imm , bx . cx () . type_isize ()) ; bx . bitcast (int_imm , to_backend_ty) } } ; debug_assert_eq ! (bx . cx () . val_ty (imm) , to_backend_ty) ; assume_scalar_range (bx , imm , to_scalar , to_backend_ty , Some (& from_scalar)) ; imm = bx . to_immediate_scalar (imm , to_scalar) ; imm }
    };
}

transmute_scalar!()