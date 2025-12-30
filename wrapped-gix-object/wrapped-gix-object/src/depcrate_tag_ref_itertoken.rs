// Generated macro for Token (enum)
macro_rules! Depcrate_tag_ref_iterToken {
() => {
// Module: crate::tag::ref_iter
// Provides: {"Token"}
// Dependencies: {}
# [doc = " A token returned by the [tag iterator][TagRefIter]."] # [allow (missing_docs)] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub enum Token < 'a > { Target { id : ObjectId , } , TargetKind (Kind) , Name (& 'a BStr) , Tagger (Option < gix_actor :: SignatureRef < 'a > >) , Body { message : & 'a BStr , pgp_signature : Option < & 'a BStr > , } , }
};
}
