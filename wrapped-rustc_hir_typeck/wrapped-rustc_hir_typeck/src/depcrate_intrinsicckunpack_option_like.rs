// Generated macro for unpack_option_like (function)
macro_rules! Depcrate_intrinsicckunpack_option_like {
() => {
// Module: crate::intrinsicck
// Provides: {"unpack_option_like"}
// Dependencies: {}
# [doc = " If the type is `Option<T>`, it will return `T`, otherwise"] # [doc = " the type itself. Works on most `Option`-like types."] fn unpack_option_like < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Ty < 'tcx > { let ty :: Adt (def , args) = * ty . kind () else { return ty } ; if def . variants () . len () == 2 && ! def . repr () . c () && def . repr () . int . is_none () { let data_idx ; let one = VariantIdx :: new (1) ; let zero = VariantIdx :: ZERO ; if def . variant (zero) . fields . is_empty () { data_idx = one ; } else if def . variant (one) . fields . is_empty () { data_idx = zero ; } else { return ty ; } if def . variant (data_idx) . fields . len () == 1 { return def . variant (data_idx) . single_field () . ty (tcx , args) ; } } ty }
};
}
