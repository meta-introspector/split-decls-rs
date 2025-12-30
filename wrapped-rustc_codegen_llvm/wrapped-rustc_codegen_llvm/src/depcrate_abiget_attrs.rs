// Generated macro for get_attrs (function)
macro_rules! Depcrate_abiget_attrs {
() => {
// Module: crate::abi
// Provides: {"get_attrs"}
// Dependencies: {}
fn get_attrs < 'll > (this : & ArgAttributes , cx : & CodegenCx < 'll , '_ >) -> SmallVec < [& 'll Attribute ; 8] > { let mut regular = this . regular ; let mut attrs = SmallVec :: new () ; for (attr , llattr) in ABI_AFFECTING_ATTRIBUTES { if regular . contains (attr) { attrs . push (llattr . create_attr (cx . llcx)) ; } } if let Some (align) = this . pointee_align { attrs . push (llvm :: CreateAlignmentAttr (cx . llcx , align . bytes ())) ; } match this . arg_ext { ArgExtension :: None => { } ArgExtension :: Zext => attrs . push (llvm :: AttributeKind :: ZExt . create_attr (cx . llcx)) , ArgExtension :: Sext => attrs . push (llvm :: AttributeKind :: SExt . create_attr (cx . llcx)) , } if cx . sess () . opts . optimize != config :: OptLevel :: No { let deref = this . pointee_size . bytes () ; if deref != 0 { if regular . contains (ArgAttribute :: NonNull) { attrs . push (llvm :: CreateDereferenceableAttr (cx . llcx , deref)) ; } else { attrs . push (llvm :: CreateDereferenceableOrNullAttr (cx . llcx , deref)) ; } regular -= ArgAttribute :: NonNull ; } for (attr , llattr) in OPTIMIZATION_ATTRIBUTES { if regular . contains (attr) { if (attr == ArgAttribute :: CapturesReadOnly || attr == ArgAttribute :: CapturesAddress) && llvm_util :: get_version () < (21 , 0 , 0) { continue ; } attrs . push (llattr . create_attr (cx . llcx)) ; } } } else if cx . tcx . sess . opts . unstable_opts . sanitizer . contains (SanitizerSet :: MEMORY) { if regular . contains (ArgAttribute :: NoUndef) { attrs . push (llvm :: AttributeKind :: NoUndef . create_attr (cx . llcx)) ; } } attrs }
};
}
