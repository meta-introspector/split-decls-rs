// Generated macro for tag_base_type_opt (function)
macro_rules! Depcrate_debuginfotag_base_type_opt {
() => {
// Module: crate::debuginfo
// Provides: {"tag_base_type_opt"}
// Dependencies: {}
fn tag_base_type_opt < 'tcx > (tcx : TyCtxt < 'tcx > , enum_type_and_layout : TyAndLayout < 'tcx > ,) -> Option < Ty < 'tcx > > { assert ! (match enum_type_and_layout . ty . kind () { ty :: Coroutine (..) => true , ty :: Adt (adt_def , _) => adt_def . is_enum () , _ => false , }) ; match enum_type_and_layout . layout . variants () { Variants :: Single { .. } | Variants :: Empty => None , Variants :: Multiple { tag_encoding : TagEncoding :: Niche { .. } , tag , .. } => { Some (match tag . primitive () { Primitive :: Int (t , _) => t , Primitive :: Float (f) => Integer :: from_size (f . size ()) . unwrap () , Primitive :: Pointer (_) => { tcx . data_layout . ptr_sized_integer () } } . to_ty (tcx , false) ,) } Variants :: Multiple { tag_encoding : TagEncoding :: Direct , tag , .. } => { Some (tag . primitive () . to_ty (tcx)) } } }
};
}
