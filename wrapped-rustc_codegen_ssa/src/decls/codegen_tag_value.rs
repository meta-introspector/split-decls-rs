macro_rules! deps {
    () => {
        UninhabitedVariantError!();
    };
}

macro_rules! codegen_tag_value {
    () => {
        deps!();
        # [doc = " Calculates the value that needs to be stored to mark the discriminant."] # [doc = ""] # [doc = " This might be `None` for a `struct` or a niched variant (like `Some(&3)`)."] # [doc = ""] # [doc = " If it's `Some`, it returns the value to store and the field in which to"] # [doc = " store it. Note that this value is *not* the same as the discriminant, in"] # [doc = " general, as it might be a niche value or have a different size."] # [doc = ""] # [doc = " It might also be an `Err` because the variant is uninhabited."] pub (super) fn codegen_tag_value < 'tcx , V > (cx : & impl CodegenMethods < 'tcx , Value = V > , variant_index : VariantIdx , layout : TyAndLayout < 'tcx > ,) -> Result < Option < (FieldIdx , V) > , UninhabitedVariantError > { if layout . for_variant (cx , variant_index) . is_uninhabited () { return Err (UninhabitedVariantError) ; } Ok (match layout . variants { Variants :: Empty => unreachable ! ("we already handled uninhabited types") , Variants :: Single { index } => { assert_eq ! (index , variant_index) ; None } Variants :: Multiple { tag_encoding : TagEncoding :: Direct , tag_field , .. } => { let discr = layout . ty . discriminant_for_variant (cx . tcx () , variant_index) ; let to = discr . unwrap () . val ; let tag_layout = layout . field (cx , tag_field . as_usize ()) ; let tag_llty = cx . immediate_backend_type (tag_layout) ; let imm = cx . const_uint_big (tag_llty , to) ; Some ((tag_field , imm)) } Variants :: Multiple { tag_encoding : TagEncoding :: Niche { untagged_variant , ref niche_variants , niche_start } , tag_field , .. } => { if variant_index != untagged_variant { let niche_layout = layout . field (cx , tag_field . as_usize ()) ; let niche_llty = cx . immediate_backend_type (niche_layout) ; let BackendRepr :: Scalar (scalar) = niche_layout . backend_repr else { bug ! ("expected a scalar placeref for the niche") ; } ; let niche_value = variant_index . as_u32 () - niche_variants . start () . as_u32 () ; let niche_value = (niche_value as u128) . wrapping_add (niche_start) ; let niche_value = niche_value & niche_layout . size . unsigned_int_max () ; let niche_llval = cx . scalar_to_backend (Scalar :: from_uint (niche_value , niche_layout . size) , scalar , niche_llty ,) ; Some ((tag_field , niche_llval)) } else { None } } }) }
    };
}

codegen_tag_value!()