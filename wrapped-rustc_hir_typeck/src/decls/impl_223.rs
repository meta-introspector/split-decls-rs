macro_rules! deps {
    () => {
        Declaration!();
        DeclOrigin!();
    };
}

macro_rules! impl_223 {
    () => {
        deps!();
        impl < 'a > From < & 'a hir :: LetStmt < 'a > > for Declaration < 'a > { fn from (local : & 'a hir :: LetStmt < 'a >) -> Self { let hir :: LetStmt { hir_id , super_ : _ , pat , ty , span , init , els , source : _ } = * local ; Declaration { hir_id , pat , ty , span , init , origin : DeclOrigin :: LocalDecl { els } } } }
    };
}

impl_223!();