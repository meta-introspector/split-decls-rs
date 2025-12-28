macro_rules! deps {
    () => {
        BindingMode!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl BindingMode { fn convert (annotation : BindingAnnotation) -> BindingMode { match annotation { BindingAnnotation :: Unannotated | BindingAnnotation :: Mutable => BindingMode :: Move , BindingAnnotation :: Ref => BindingMode :: Ref (Mutability :: Not) , BindingAnnotation :: RefMut => BindingMode :: Ref (Mutability :: Mut) , } } }
    };
}

impl_54!();