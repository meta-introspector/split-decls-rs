macro_rules! deps {
    () => {
        ConstArg!();
        GenericArg!();
        Ty!();
        InferArg!();
    };
}

macro_rules! InferKind {
    () => {
        deps!();
        # [doc = " We track whether an infer var is from a [`Ty`], [`ConstArg`], or [`GenericArg`] so that"] # [doc = " HIR visitors overriding [`Visitor::visit_infer`] can determine what kind of infer is being visited"] pub enum InferKind < 'hir > { Ty (& 'hir Ty < 'hir >) , Const (& 'hir ConstArg < 'hir >) , Ambig (& 'hir InferArg) , }
    };
}

InferKind!();