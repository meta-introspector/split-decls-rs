macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! struct_llfields {
    () => {
        deps!();
        fn struct_llfields < 'a , 'tcx > (cx : & CodegenCx < 'a , 'tcx > , layout : TyAndLayout < 'tcx > ,) -> (Vec < & 'a Type > , bool) { debug ! ("struct_llfields: {:#?}" , layout) ; let field_count = layout . fields . count () ; let mut packed = false ; let mut offset = Size :: ZERO ; let mut prev_effective_align = layout . align . abi ; let mut result : Vec < _ > = Vec :: with_capacity (1 + field_count * 2) ; for i in layout . fields . index_by_increasing_offset () { let target_offset = layout . fields . offset (i as usize) ; let field = layout . field (cx , i) ; let effective_field_align = layout . align . abi . min (field . align . abi) . restrict_for_offset (target_offset) ; packed |= effective_field_align < field . align . abi ; debug ! ("struct_llfields: {}: {:?} offset: {:?} target_offset: {:?} \
                effective_field_align: {}" , i , field , offset , target_offset , effective_field_align . bytes ()) ; assert ! (target_offset >= offset) ; let padding = target_offset - offset ; if padding != Size :: ZERO { let padding_align = prev_effective_align . min (effective_field_align) ; assert_eq ! (offset . align_to (padding_align) + padding , target_offset) ; result . push (cx . type_padding_filler (padding , padding_align)) ; debug ! ("    padding before: {:?}" , padding) ; } result . push (field . llvm_type (cx)) ; offset = target_offset + field . size ; prev_effective_align = effective_field_align ; } if layout . is_sized () && field_count > 0 { if offset > layout . size { bug ! ("layout: {:#?} stride: {:?} offset: {:?}" , layout , layout . size , offset) ; } let padding = layout . size - offset ; if padding != Size :: ZERO { let padding_align = prev_effective_align ; assert_eq ! (offset . align_to (padding_align) + padding , layout . size) ; debug ! ("struct_llfields: pad_bytes: {:?} offset: {:?} stride: {:?}" , padding , offset , layout . size) ; result . push (cx . type_padding_filler (padding , padding_align)) ; } } else { debug ! ("struct_llfields: offset: {:?} stride: {:?}" , offset , layout . size) ; } (result , packed) }
    };
}

struct_llfields!();