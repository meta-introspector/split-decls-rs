// Generated macro for probestack_attr (function)
macro_rules! Depcrate_attributesprobestack_attr {
() => {
// Module: crate::attributes
// Provides: {"probestack_attr"}
// Dependencies: {}
fn probestack_attr < 'll > (cx : & CodegenCx < 'll , '_ >) -> Option < & 'll Attribute > { if cx . sess () . opts . unstable_opts . sanitizer . intersects (SanitizerSet :: ADDRESS | SanitizerSet :: THREAD) { return None ; } if cx . sess () . opts . cg . profile_generate . enabled () { return None ; } let attr_value = match cx . sess () . target . stack_probes { StackProbeType :: None => return None , StackProbeType :: Inline => "inline-asm" , StackProbeType :: Call => & mangle_internal_symbol (cx . tcx , "__rust_probestack") , StackProbeType :: InlineOrCall { min_llvm_version_for_inline } => { if llvm_util :: get_version () < min_llvm_version_for_inline { & mangle_internal_symbol (cx . tcx , "__rust_probestack") } else { "inline-asm" } } } ; Some (llvm :: CreateAttrStringValue (cx . llcx , "probe-stack" , attr_value)) }
};
}
