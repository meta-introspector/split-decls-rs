macro_rules! deps {
    () => {
        Variants!();
        Layout!();
        TagEncoding!();
        HirDatabase!();
    };
}

macro_rules! detect_variant_from_bytes {
    () => {
        deps!();
        pub (crate) fn detect_variant_from_bytes < 'a > (layout : & 'a Layout , db : & dyn HirDatabase , target_data_layout : & TargetDataLayout , b : & [u8] , e : EnumId ,) -> Option < (EnumVariantId , & 'a Layout) > { let (var_id , var_layout) = match & layout . variants { hir_def :: layout :: Variants :: Empty => unreachable ! () , hir_def :: layout :: Variants :: Single { index } => { (e . enum_variants (db) . variants [index . 0] . 0 , layout) } hir_def :: layout :: Variants :: Multiple { tag , tag_encoding , variants , .. } => { let size = tag . size (target_data_layout) . bytes_usize () ; let offset = layout . fields . offset (0) . bytes_usize () ; let tag = i128 :: from_le_bytes (pad16 (& b [offset .. offset + size] , false)) ; match tag_encoding { TagEncoding :: Direct => { let (var_idx , layout) = variants . iter_enumerated () . find_map (| (var_idx , v) | { let def = e . enum_variants (db) . variants [var_idx . 0] . 0 ; (db . const_eval_discriminant (def) == Ok (tag)) . then_some ((def , v)) }) ? ; (var_idx , layout) } TagEncoding :: Niche { untagged_variant , niche_start , .. } => { let candidate_tag = tag . wrapping_sub (* niche_start as i128) as usize ; let variant = variants . iter_enumerated () . map (| (x , _) | x) . filter (| x | x != untagged_variant) . nth (candidate_tag) . unwrap_or (* untagged_variant) ; (e . enum_variants (db) . variants [variant . 0] . 0 , & variants [variant]) } } } } ; Some ((var_id , var_layout)) }
    };
}

detect_variant_from_bytes!()