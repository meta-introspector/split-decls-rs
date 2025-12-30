// Generated macro for check_simd (function)
macro_rules! Depcrate_check_checkcheck_simd {
() => {
// Module: crate::check::check
// Provides: {"check_simd"}
// Dependencies: {}
fn check_simd (tcx : TyCtxt < '_ > , sp : Span , def_id : LocalDefId) { let t = tcx . type_of (def_id) . instantiate_identity () ; if let ty :: Adt (def , args) = t . kind () && def . is_struct () { let fields = & def . non_enum_variant () . fields ; if fields . is_empty () { struct_span_code_err ! (tcx . dcx () , sp , E0075 , "SIMD vector cannot be empty") . emit () ; return ; } let array_field = & fields [FieldIdx :: ZERO] ; let array_ty = array_field . ty (tcx , args) ; let ty :: Array (element_ty , len_const) = array_ty . kind () else { struct_span_code_err ! (tcx . dcx () , sp , E0076 , "SIMD vector's only field must be an array") . with_span_label (tcx . def_span (array_field . did) , "not an array") . emit () ; return ; } ; if let Some (second_field) = fields . get (FieldIdx :: ONE) { struct_span_code_err ! (tcx . dcx () , sp , E0075 , "SIMD vector cannot have multiple fields") . with_span_label (tcx . def_span (second_field . did) , "excess field") . emit () ; return ; } if let Some (len) = len_const . try_to_target_usize (tcx) { if len == 0 { struct_span_code_err ! (tcx . dcx () , sp , E0075 , "SIMD vector cannot be empty") . emit () ; return ; } else if len > MAX_SIMD_LANES { struct_span_code_err ! (tcx . dcx () , sp , E0075 , "SIMD vector cannot have more than {MAX_SIMD_LANES} elements" ,) . emit () ; return ; } } match element_ty . kind () { ty :: Param (_) => () , ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: RawPtr (_ , _) => () , _ => { struct_span_code_err ! (tcx . dcx () , sp , E0077 , "SIMD vector element type should be a \
                        primitive scalar (integer/float/pointer) type") . emit () ; return ; } } } }
};
}
