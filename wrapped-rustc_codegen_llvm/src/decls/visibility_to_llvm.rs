macro_rules! deps {
    () => {
        Visibility!();
    };
}

macro_rules! visibility_to_llvm {
    () => {
        deps!();
        pub (crate) fn visibility_to_llvm (linkage : Visibility) -> llvm :: Visibility { match linkage { Visibility :: Default => llvm :: Visibility :: Default , Visibility :: Hidden => llvm :: Visibility :: Hidden , Visibility :: Protected => llvm :: Visibility :: Protected , } }
    };
}

visibility_to_llvm!();