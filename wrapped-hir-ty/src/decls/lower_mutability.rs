macro_rules! lower_mutability {
    () => {
        pub (crate) fn lower_mutability (m : hir_def :: type_ref :: Mutability) -> Mutability { match m { hir_def :: type_ref :: Mutability :: Shared => Mutability :: Not , hir_def :: type_ref :: Mutability :: Mut => Mutability :: Mut , } }
    };
}

lower_mutability!();