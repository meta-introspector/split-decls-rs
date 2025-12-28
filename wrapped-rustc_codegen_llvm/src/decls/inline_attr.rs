macro_rules! deps {
    () => {
        CodegenCx!();
        AttributeKind!();
    };
}

macro_rules! inline_attr {
    () => {
        deps!();
        # [doc = " Get LLVM attribute for the provided inline heuristic."] pub (crate) fn inline_attr < 'll , 'tcx > (cx : & CodegenCx < 'll , 'tcx > , instance : ty :: Instance < 'tcx > ,) -> Option < & 'll Attribute > { let codegen_fn_attrs = cx . tcx . codegen_fn_attrs (instance . def_id ()) ; let inline = match (codegen_fn_attrs . inline , & codegen_fn_attrs . optimize) { (_ , OptimizeAttr :: DoNotOptimize) => InlineAttr :: Never , (InlineAttr :: None , _) if instance . def . requires_inline (cx . tcx) => InlineAttr :: Hint , (inline , _) => inline , } ; if ! cx . tcx . sess . opts . unstable_opts . inline_llvm { return Some (AttributeKind :: NoInline . create_attr (cx . llcx)) ; } match inline { InlineAttr :: Hint => Some (AttributeKind :: InlineHint . create_attr (cx . llcx)) , InlineAttr :: Always | InlineAttr :: Force { .. } => { Some (AttributeKind :: AlwaysInline . create_attr (cx . llcx)) } InlineAttr :: Never => { if cx . sess () . target . arch != "amdgpu" { Some (AttributeKind :: NoInline . create_attr (cx . llcx)) } else { None } } InlineAttr :: None => None , } }
    };
}

inline_attr!();