// Generated macro for is_struct_repr_transparent (function)
macro_rules! Depcrate_volatile_compositesis_struct_repr_transparent {
() => {
// Module: crate::volatile_composites
// Provides: {"is_struct_repr_transparent"}
// Dependencies: {}
# [doc = " `#[repr(transparent)]` structures are also OK if the only non-zero"] # [doc = " sized field contains a volatile-safe type."] fn is_struct_repr_transparent < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { if let ty :: Adt (adt_def , args) = ty . kind () && adt_def . is_struct () && adt_def . repr () . transparent () && let [fieldty] = adt_def . all_fields () . filter_map (| field | { let fty = field . ty (cx . tcx , args) ; if is_zero_sized_ty (cx , fty) { None } else { Some (fty) } }) . collect :: < Vec < _ > > () . as_slice () { is_volatile_safe_ty (cx , * fieldty) } else { false } }
};
}
