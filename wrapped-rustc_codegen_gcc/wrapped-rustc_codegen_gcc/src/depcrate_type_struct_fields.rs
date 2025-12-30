// Generated macro for struct_fields (function)
macro_rules! Depcrate_type_struct_fields {
() => {
// Module: crate::type_
// Provides: {"struct_fields"}
// Dependencies: {}
pub fn struct_fields < 'gcc , 'tcx > (cx : & CodegenCx < 'gcc , 'tcx > , layout : TyAndLayout < 'tcx > ,) -> (Vec < Type < 'gcc > > , bool) { let field_count = layout . fields . count () ; let mut packed = false ; let mut offset = Size :: ZERO ; let mut prev_effective_align = layout . align . abi ; let mut result : Vec < _ > = Vec :: with_capacity (1 + field_count * 2) ; for i in layout . fields . index_by_increasing_offset () { let target_offset = layout . fields . offset (i) ; let field = layout . field (cx , i) ; let effective_field_align = layout . align . abi . min (field . align . abi) . restrict_for_offset (target_offset) ; packed |= effective_field_align < field . align . abi ; assert ! (target_offset >= offset) ; let padding = target_offset - offset ; let padding_align = prev_effective_align . min (effective_field_align) ; assert_eq ! (offset . align_to (padding_align) + padding , target_offset) ; result . push (cx . type_padding_filler (padding , padding_align)) ; result . push (field . gcc_type (cx)) ; offset = target_offset + field . size ; prev_effective_align = effective_field_align ; } if layout . is_sized () && field_count > 0 { if offset > layout . size { bug ! ("layout: {:#?} stride: {:?} offset: {:?}" , layout , layout . size , offset) ; } let padding = layout . size - offset ; let padding_align = prev_effective_align ; assert_eq ! (offset . align_to (padding_align) + padding , layout . size) ; result . push (cx . type_padding_filler (padding , padding_align)) ; assert_eq ! (result . len () , 1 + field_count * 2) ; } (result , packed) }
};
}
