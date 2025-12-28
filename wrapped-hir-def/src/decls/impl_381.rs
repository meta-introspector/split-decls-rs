macro_rules! deps {
    () => {
        ModuleSource!();
    };
}

macro_rules! impl_381 {
    () => {
        deps!();
        impl ModuleSource { pub fn node (& self) -> SyntaxNode { match self { ModuleSource :: SourceFile (it) => it . syntax () . clone () , ModuleSource :: Module (it) => it . syntax () . clone () , ModuleSource :: BlockExpr (it) => it . syntax () . clone () , } } }
    };
}

impl_381!()