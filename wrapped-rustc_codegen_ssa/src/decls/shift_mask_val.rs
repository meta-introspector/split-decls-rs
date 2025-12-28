macro_rules! deps {
    () => {
        TypeKind!();
        BuilderMethods!();
    };
}

macro_rules! shift_mask_val {
    () => {
        deps!();
        pub (crate) fn shift_mask_val < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , llty : Bx :: Type , mask_llty : Bx :: Type , invert : bool ,) -> Bx :: Value { let kind = bx . type_kind (llty) ; match kind { TypeKind :: Integer => { let val = bx . int_width (llty) - 1 ; if invert { bx . const_int (mask_llty , ! val as i64) } else { bx . const_uint (mask_llty , val) } } TypeKind :: Vector => { let mask = shift_mask_val (bx , bx . element_type (llty) , bx . element_type (mask_llty) , invert) ; bx . vector_splat (bx . vector_length (mask_llty) , mask) } _ => bug ! ("shift_mask_val: expected Integer or Vector, found {:?}" , kind) , } }
    };
}

shift_mask_val!()