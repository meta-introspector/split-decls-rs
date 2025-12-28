macro_rules! path_kind_len {
    () => {
        fn path_kind_len (kind : PathKind) -> usize { match kind { PathKind :: Plain => 0 , PathKind :: Super (0) => 4 , PathKind :: Super (s) => s as usize * 5 , PathKind :: Crate => 5 , PathKind :: Abs => 2 , PathKind :: DollarCrate (_) => 0 , } }
    };
}

path_kind_len!();