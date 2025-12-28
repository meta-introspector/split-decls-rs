macro_rules! deps {
    () => {
        BindingAnnotation!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl BindingAnnotation { pub fn new (is_mutable : bool , is_ref : bool) -> Self { match (is_mutable , is_ref) { (true , true) => BindingAnnotation :: RefMut , (false , true) => BindingAnnotation :: Ref , (true , false) => BindingAnnotation :: Mutable , (false , false) => BindingAnnotation :: Unannotated , } } }
    };
}

impl_247!();