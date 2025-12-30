// Generated macro for prop_append_dict (function)
macro_rules! Depcrate_leavesprop_append_dict {
() => {
// Module: crate::leaves
// Provides: {"prop_append_dict"}
// Dependencies: {}
pub fn prop_append_dict < 'v , M : MethodType < D > + 'v , D : DataType + 'v , I : Iterator < Item = & 'v Property < M , D > > > (iter : & mut arg :: IterAppend , mut props : I , minfo : & MethodInfo < M , D >) -> Result < () , MethodErr > { let mut result = Ok (()) ; iter . append_dict (& Signature :: make :: < & str > () , & Signature :: make :: < arg :: Variant < bool > > () , | subiter | loop { let p = if let Some (p) = props . next () { p } else { return } ; if p . can_get () . is_err () { continue ; } let pinfo = minfo . to_prop_info (minfo . iface , p) ; subiter . append_dict_entry (| mut entryiter | { entryiter . append (& * p . get_name ()) ; result = p . get_as_variant (& mut entryiter , & pinfo) ; }) ; if result . is_err () { return } ; }) ; result }
};
}
