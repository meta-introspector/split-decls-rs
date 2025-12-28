macro_rules! deps {
    () => {
        AnnotationKind!();
        Loc!();
    };
}

macro_rules! MultilineAnnotation {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialOrd , Ord , PartialEq , Eq)] pub (crate) struct MultilineAnnotation < 'a > { pub depth : usize , pub start : Loc , pub end : Loc , pub kind : AnnotationKind , pub label : Option < Cow < 'a , str > > , pub overlaps_exactly : bool , pub highlight_source : bool , }
    };
}

MultilineAnnotation!();