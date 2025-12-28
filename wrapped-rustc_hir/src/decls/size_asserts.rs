macro_rules! deps {
    () => {
        GenericArg!();
        LetStmt!();
        Res!();
        Block!();
        GenericBound!();
        Item!();
        StmtKind!();
        TraitItemKind!();
        FnDecl!();
        Ty!();
        TraitItem!();
        Generics!();
        ForeignItemKind!();
        Pat!();
        ExprKind!();
        PathSegment!();
        TyKind!();
        Body!();
        Path!();
        Stmt!();
        TraitImplHeader!();
        PatKind!();
        ImplItem!();
        Expr!();
        Impl!();
        ForeignItem!();
        Param!();
        ImplItemKind!();
        ItemKind!();
        QPath!();
    };
}

macro_rules! size_asserts {
    () => {
        deps!();
        # [cfg (target_pointer_width = "64")] mod size_asserts { use rustc_data_structures :: static_assert_size ; use super :: * ; static_assert_size ! (Block <'_ >, 48) ; static_assert_size ! (Body <'_ >, 24) ; static_assert_size ! (Expr <'_ >, 64) ; static_assert_size ! (ExprKind <'_ >, 48) ; static_assert_size ! (FnDecl <'_ >, 40) ; static_assert_size ! (ForeignItem <'_ >, 96) ; static_assert_size ! (ForeignItemKind <'_ >, 56) ; static_assert_size ! (GenericArg <'_ >, 16) ; static_assert_size ! (GenericBound <'_ >, 64) ; static_assert_size ! (Generics <'_ >, 56) ; static_assert_size ! (Impl <'_ >, 40) ; static_assert_size ! (ImplItem <'_ >, 88) ; static_assert_size ! (ImplItemKind <'_ >, 40) ; static_assert_size ! (Item <'_ >, 88) ; static_assert_size ! (ItemKind <'_ >, 64) ; static_assert_size ! (LetStmt <'_ >, 72) ; static_assert_size ! (Param <'_ >, 32) ; static_assert_size ! (Pat <'_ >, 80) ; static_assert_size ! (PatKind <'_ >, 56) ; static_assert_size ! (Path <'_ >, 40) ; static_assert_size ! (PathSegment <'_ >, 48) ; static_assert_size ! (QPath <'_ >, 24) ; static_assert_size ! (Res , 12) ; static_assert_size ! (Stmt <'_ >, 32) ; static_assert_size ! (StmtKind <'_ >, 16) ; static_assert_size ! (TraitImplHeader <'_ >, 48) ; static_assert_size ! (TraitItem <'_ >, 88) ; static_assert_size ! (TraitItemKind <'_ >, 48) ; static_assert_size ! (Ty <'_ >, 48) ; static_assert_size ! (TyKind <'_ >, 32) ; }
    };
}

size_asserts!()