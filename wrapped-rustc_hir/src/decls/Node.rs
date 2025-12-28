macro_rules! deps {
    () => {
        VariantData!();
        WherePredicate!();
        Param!();
        PreciseCapturingNonLifetimeArg!();
        Crate!();
        Pat!();
        ExprField!();
        ForeignItem!();
        PatExpr!();
        Arm!();
        PatField!();
        GenericParam!();
        AssocItemConstraint!();
        TyPat!();
        Mod!();
        TraitItem!();
        TraitRef!();
        Lifetime!();
        ConstArg!();
        Ty!();
        ConstBlock!();
        InferArg!();
        OpaqueTy!();
        Item!();
        Variant!();
        Expr!();
        Stmt!();
        PathSegment!();
        Block!();
        LetStmt!();
        FieldDef!();
        ImplItem!();
        AnonConst!();
    };
}

macro_rules! Node {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum Node < 'hir > { Param (& 'hir Param < 'hir >) , Item (& 'hir Item < 'hir >) , ForeignItem (& 'hir ForeignItem < 'hir >) , TraitItem (& 'hir TraitItem < 'hir >) , ImplItem (& 'hir ImplItem < 'hir >) , Variant (& 'hir Variant < 'hir >) , Field (& 'hir FieldDef < 'hir >) , AnonConst (& 'hir AnonConst) , ConstBlock (& 'hir ConstBlock) , ConstArg (& 'hir ConstArg < 'hir >) , Expr (& 'hir Expr < 'hir >) , ExprField (& 'hir ExprField < 'hir >) , Stmt (& 'hir Stmt < 'hir >) , PathSegment (& 'hir PathSegment < 'hir >) , Ty (& 'hir Ty < 'hir >) , AssocItemConstraint (& 'hir AssocItemConstraint < 'hir >) , TraitRef (& 'hir TraitRef < 'hir >) , OpaqueTy (& 'hir OpaqueTy < 'hir >) , TyPat (& 'hir TyPat < 'hir >) , Pat (& 'hir Pat < 'hir >) , PatField (& 'hir PatField < 'hir >) , # [doc = " Needed as its own node with its own HirId for tracking"] # [doc = " the unadjusted type of literals within patterns"] # [doc = " (e.g. byte str literals not being of slice type)."] PatExpr (& 'hir PatExpr < 'hir >) , Arm (& 'hir Arm < 'hir >) , Block (& 'hir Block < 'hir >) , LetStmt (& 'hir LetStmt < 'hir >) , # [doc = " `Ctor` refers to the constructor of an enum variant or struct. Only tuple or unit variants"] # [doc = " with synthesized constructors."] Ctor (& 'hir VariantData < 'hir >) , Lifetime (& 'hir Lifetime) , GenericParam (& 'hir GenericParam < 'hir >) , Crate (& 'hir Mod < 'hir >) , Infer (& 'hir InferArg) , WherePredicate (& 'hir WherePredicate < 'hir >) , PreciseCapturingNonLifetimeArg (& 'hir PreciseCapturingNonLifetimeArg) , Synthetic , Err (Span) , }
    };
}

Node!();