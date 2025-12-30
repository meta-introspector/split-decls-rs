// Generated macro for impl_1295 (impl)
macro_rules! Depcrate_typesimpl_1295 {
() => {
// Module: crate::types
// Provides: {"impl_1295"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for VariantSizeDifferences { fn check_item (& mut self , cx : & LateContext < '_ > , it : & hir :: Item < '_ >) { if let hir :: ItemKind :: Enum (_ , _ , ref enum_definition) = it . kind { let t = cx . tcx . type_of (it . owner_id) . instantiate_identity () ; let ty = cx . tcx . erase_and_anonymize_regions (t) ; let Ok (layout) = cx . layout_of (ty) else { return } ; let Variants :: Multiple { tag_encoding : TagEncoding :: Direct , tag , variants , .. } = & layout . variants else { return ; } ; let tag_size = tag . size (& cx . tcx) . bytes () ; debug ! ("enum `{}` is {} bytes large with layout:\n{:#?}" , t , layout . size . bytes () , layout) ; let (largest , slargest , largest_index) = iter :: zip (enum_definition . variants , variants) . map (| (variant , variant_layout) | { let bytes = variant_layout . size . bytes () . saturating_sub (tag_size) ; debug ! ("- variant `{}` is {} bytes large" , variant . ident , bytes) ; bytes }) . enumerate () . fold ((0 , 0 , 0) , | (l , s , li) , (idx , size) | { if size > l { (size , l , idx) } else if size > s { (l , size , li) } else { (l , s , li) } }) ; if largest > slargest * 3 && slargest > 0 { cx . emit_span_lint (VARIANT_SIZE_DIFFERENCES , enum_definition . variants [largest_index] . span , VariantSizeDifferencesDiag { largest } ,) ; } } } }
};
}
