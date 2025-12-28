macro_rules! deps {
    () => {
        Semantics!();
        DeclarationLocation!();
    };
}

macro_rules! impl_176 {
    () => {
        deps!();
        impl DeclarationLocation { pub fn syntax < DB : HirDatabase > (& self , sema : & Semantics < '_ , DB >) -> SyntaxNode { let root = sema . parse_or_expand (self . hir_file_id) ; self . ptr . to_node (& root) } }
    };
}

impl_176!()