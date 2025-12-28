macro_rules! deps {
    () => {
        Expr!();
    };
}

macro_rules! StructTailExpr {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum StructTailExpr < 'hir > { # [doc = " A struct expression where all the fields are explicitly enumerated: `Foo { a, b }`."] None , # [doc = " A struct expression with a \"base\", an expression of the same type as the outer struct that"] # [doc = " will be used to populate any fields not explicitly mentioned: `Foo { ..base }`"] Base (& 'hir Expr < 'hir >) , # [doc = " A struct expression with a `..` tail but no \"base\" expression. The values from the struct"] # [doc = " fields' default values will be used to populate any fields not explicitly mentioned:"] # [doc = " `Foo { .. }`."] DefaultFields (Span) , }
    };
}

StructTailExpr!()