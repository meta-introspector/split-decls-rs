macro_rules! deps {
    () => {
        LineAnnotation!();
        AnnotationKind!();
        LineAnnotationType!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl LineAnnotation < '_ > { pub (crate) fn is_primary (& self) -> bool { self . kind == AnnotationKind :: Primary } # [doc = " Whether this annotation is a vertical line placeholder."] pub (crate) fn is_line (& self) -> bool { matches ! (self . annotation_type , LineAnnotationType :: MultilineLine (_)) } # [doc = " Length of this annotation as displayed in the stderr output"] pub (crate) fn len (& self) -> usize { self . end . display . abs_diff (self . start . display) } pub (crate) fn has_label (& self) -> bool { if let Some (label) = & self . label { ! label . is_empty () } else { false } } pub (crate) fn takes_space (& self) -> bool { matches ! (self . annotation_type , LineAnnotationType :: MultilineStart (_) | LineAnnotationType :: MultilineEnd (_)) } }
    };
}

impl_42!()