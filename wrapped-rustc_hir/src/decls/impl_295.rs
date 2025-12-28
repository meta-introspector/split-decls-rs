macro_rules! deps {
    () => {
        Ty!();
        FnRetTy!();
    };
}

macro_rules! impl_295 {
    () => {
        deps!();
        impl < 'hir > FnRetTy < 'hir > { # [inline] pub fn span (& self) -> Span { match * self { Self :: DefaultReturn (span) => span , Self :: Return (ref ty) => ty . span , } } pub fn is_suggestable_infer_ty (& self) -> Option < & 'hir Ty < 'hir > > { if let Self :: Return (ty) = self && ty . is_suggestable_infer_ty () { return Some (* ty) ; } None } }
    };
}

impl_295!()