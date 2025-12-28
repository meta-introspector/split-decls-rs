macro_rules! deps {
    () => {
        SubregionOrigin!();
    };
}

macro_rules! impl_277 {
    () => {
        deps!();
        impl < 'tcx > SubregionOrigin < 'tcx > { pub fn span (& self) -> Span { match * self { SubregionOrigin :: Subtype (ref a) => a . span () , SubregionOrigin :: RelateObjectBound (a) => a , SubregionOrigin :: RelateParamBound (a , ..) => a , SubregionOrigin :: RelateRegionParamBound (a , _) => a , SubregionOrigin :: Reborrow (a) => a , SubregionOrigin :: ReferenceOutlivesReferent (_ , a) => a , SubregionOrigin :: CompareImplItemObligation { span , .. } => span , SubregionOrigin :: AscribeUserTypeProvePredicate (span) => span , SubregionOrigin :: CheckAssociatedTypeBounds { ref parent , .. } => parent . span () , } } pub fn from_obligation_cause < F > (cause : & traits :: ObligationCause < 'tcx > , default : F) -> Self where F : FnOnce () -> Self , { match * cause . code () { traits :: ObligationCauseCode :: ReferenceOutlivesReferent (ref_type) => { SubregionOrigin :: ReferenceOutlivesReferent (ref_type , cause . span) } traits :: ObligationCauseCode :: CompareImplItem { impl_item_def_id , trait_item_def_id , kind : _ , } => SubregionOrigin :: CompareImplItemObligation { span : cause . span , impl_item_def_id , trait_item_def_id , } , traits :: ObligationCauseCode :: CheckAssociatedTypeBounds { impl_item_def_id , trait_item_def_id , } => SubregionOrigin :: CheckAssociatedTypeBounds { impl_item_def_id , trait_item_def_id , parent : Box :: new (default ()) , } , traits :: ObligationCauseCode :: AscribeUserTypeProvePredicate (span) => { SubregionOrigin :: AscribeUserTypeProvePredicate (span) } traits :: ObligationCauseCode :: ObjectTypeBound (ty , _reg) => { SubregionOrigin :: RelateRegionParamBound (cause . span , Some (ty)) } _ => default () , } } }
    };
}

impl_277!()