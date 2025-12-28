macro_rules! deps {
    () => {
        TypeTrace!();
    };
}

macro_rules! SubregionOrigin {
    () => {
        deps!();
        # [doc = " The origin of a `r1 <= r2` constraint."] # [doc = ""] # [doc = " See `error_reporting` module for more details"] # [derive (Clone , Debug)] pub enum SubregionOrigin < 'tcx > { # [doc = " Arose from a subtyping relation"] Subtype (Box < TypeTrace < 'tcx > >) , # [doc = " When casting `&'a T` to an `&'b Trait` object,"] # [doc = " relating `'a` to `'b`."] RelateObjectBound (Span) , # [doc = " Some type parameter was instantiated with the given type,"] # [doc = " and that type must outlive some region."] RelateParamBound (Span , Ty < 'tcx > , Option < Span >) , # [doc = " The given region parameter was instantiated with a region"] # [doc = " that must outlive some other region."] RelateRegionParamBound (Span , Option < Ty < 'tcx > >) , # [doc = " Creating a pointer `b` to contents of another reference."] Reborrow (Span) , # [doc = " (&'a &'b T) where a >= b"] ReferenceOutlivesReferent (Ty < 'tcx > , Span) , # [doc = " Comparing the signature and requirements of an impl method against"] # [doc = " the containing trait."] CompareImplItemObligation { span : Span , impl_item_def_id : LocalDefId , trait_item_def_id : DefId , } , # [doc = " Checking that the bounds of a trait's associated type hold for a given impl."] CheckAssociatedTypeBounds { parent : Box < SubregionOrigin < 'tcx > > , impl_item_def_id : LocalDefId , trait_item_def_id : DefId , } , AscribeUserTypeProvePredicate (Span) , }
    };
}

SubregionOrigin!();