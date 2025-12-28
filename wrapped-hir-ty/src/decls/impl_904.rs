macro_rules! deps {
    () => {
        MirSpan!();
    };
}

macro_rules! impl_904 {
    () => {
        deps!();
        impl MirSpan { pub fn is_ref_span (& self , body : & Body) -> bool { match * self { MirSpan :: ExprId (expr) => matches ! (body [expr] , Expr :: Ref { .. }) , MirSpan :: BindingId (binding) => { matches ! (body [binding] . mode , BindingAnnotation :: Ref | BindingAnnotation :: RefMut) } MirSpan :: PatId (_) | MirSpan :: SelfParam | MirSpan :: Unknown => false , } } }
    };
}

impl_904!();