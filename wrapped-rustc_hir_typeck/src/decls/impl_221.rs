macro_rules! deps {
    () => {
        DeclOrigin!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        impl < 'a > DeclOrigin < 'a > { pub (super) fn try_get_else (& self) -> Option < & 'a hir :: Block < 'a > > { match self { Self :: LocalDecl { els } => * els , Self :: LetExpr => None , } } }
    };
}

impl_221!()