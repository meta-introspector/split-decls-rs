// Generated macro for try_destructure_mir_constant_for_user_output (function)
macro_rules! Depcrate_const_evaltry_destructure_mir_constant_for_user_output {
() => {
// Module: crate::const_eval
// Provides: {"try_destructure_mir_constant_for_user_output"}
// Dependencies: {}
# [instrument (skip (tcx) , level = "debug")] pub (crate) fn try_destructure_mir_constant_for_user_output < 'tcx > (tcx : TyCtxt < 'tcx > , val : mir :: ConstValue , ty : Ty < 'tcx > ,) -> Option < mir :: DestructuredConstant < 'tcx > > { let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let (ecx , op) = mk_eval_cx_for_const_val (tcx . at (rustc_span :: DUMMY_SP) , typing_env , val , ty) ? ; let (field_count , variant , down) = match ty . kind () { ty :: Array (_ , len) => (len . try_to_target_usize (tcx) ? as usize , None , op) , ty :: Adt (def , _) if def . variants () . is_empty () => { return None ; } ty :: Adt (def , _) => { let variant = ecx . read_discriminant (& op) . discard_err () ? ; let down = ecx . project_downcast (& op , variant) . discard_err () ? ; (def . variants () [variant] . fields . len () , Some (variant) , down) } ty :: Tuple (args) => (args . len () , None , op) , _ => bug ! ("cannot destructure mir constant {:?}" , val) , } ; let fields_iter = (0 .. field_count) . map (| i | { let field_op = ecx . project_field (& down , FieldIdx :: from_usize (i)) . discard_err () ? ; let val = op_to_const (& ecx , & field_op , true) ; Some ((val , field_op . layout . ty)) }) . collect :: < Option < Vec < _ > > > () ? ; let fields = tcx . arena . alloc_from_iter (fields_iter) ; Some (mir :: DestructuredConstant { variant , fields }) }
};
}
