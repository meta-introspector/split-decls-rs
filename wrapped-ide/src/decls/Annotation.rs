macro_rules! deps {
    () => {
        AnnotationKind!();
    };
}

macro_rules! Annotation {
    () => {
        deps!();
        # [derive (Debug , Hash , PartialEq , Eq)] pub struct Annotation { pub range : TextRange , pub kind : AnnotationKind , }
    };
}

Annotation!();