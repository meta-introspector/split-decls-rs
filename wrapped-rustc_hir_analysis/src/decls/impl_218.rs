macro_rules! deps {
    () => {
        HirPlaceholderCollector!();
    };
}

macro_rules! impl_218 {
    () => {
        deps!();
        impl < 'v > Visitor < 'v > for HirPlaceholderCollector { fn visit_infer (& mut self , _inf_id : HirId , inf_span : Span , kind : InferKind < 'v >) -> Self :: Result { self . spans . push (inf_span) ; if let InferKind :: Const (_) | InferKind :: Ambig (_) = kind { self . may_contain_const_infer = true ; } } }
    };
}

impl_218!()