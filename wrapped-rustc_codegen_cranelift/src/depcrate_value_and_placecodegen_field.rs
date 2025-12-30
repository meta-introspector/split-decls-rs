// Generated macro for codegen_field (function)
macro_rules! Depcrate_value_and_placecodegen_field {
() => {
// Module: crate::value_and_place
// Provides: {"codegen_field"}
// Dependencies: {}
fn codegen_field < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , base : Pointer , extra : Option < Value > , layout : TyAndLayout < 'tcx > , field : FieldIdx ,) -> (Pointer , TyAndLayout < 'tcx >) { let field_offset = layout . fields . offset (field . index ()) ; let field_layout = layout . field (& * fx , field . index ()) ; let simple = | fx : & mut FunctionCx < '_ , '_ , '_ > | { (base . offset_i64 (fx , i64 :: try_from (field_offset . bytes ()) . unwrap ()) , field_layout) } ; if field_layout . is_sized () { return simple (fx) ; } match field_layout . ty . kind () { ty :: Slice (..) | ty :: Str => simple (fx) , _ => { let unaligned_offset = field_offset . bytes () ; let (_ , mut unsized_align) = crate :: unsize :: size_and_align_of (fx , field_layout , extra) ; if let ty :: Adt (def , _) = layout . ty . kind () { if let Some (packed) = def . repr () . pack { let packed = fx . bcx . ins () . iconst (fx . pointer_type , packed . bytes () as i64) ; let cmp = fx . bcx . ins () . icmp (IntCC :: UnsignedLessThan , unsized_align , packed) ; unsized_align = fx . bcx . ins () . select (cmp , unsized_align , packed) ; } } let one = fx . bcx . ins () . iconst (fx . pointer_type , 1) ; let align_sub_1 = fx . bcx . ins () . isub (unsized_align , one) ; let and_lhs = fx . bcx . ins () . iadd_imm (align_sub_1 , unaligned_offset as i64) ; let zero = fx . bcx . ins () . iconst (fx . pointer_type , 0) ; let and_rhs = fx . bcx . ins () . isub (zero , unsized_align) ; let offset = fx . bcx . ins () . band (and_lhs , and_rhs) ; (base . offset_value (fx , offset) , field_layout) } } }
};
}
