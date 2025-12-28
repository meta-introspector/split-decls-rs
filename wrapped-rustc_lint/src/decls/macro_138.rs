macro_rules! deps {
    () => {
        DanglingPointers!();
    };
}

macro_rules! macro_138 {
    () => {
        deps!();
        impl_lint_pass ! (DanglingPointers => [DANGLING_POINTERS_FROM_TEMPORARIES , DANGLING_POINTERS_FROM_LOCALS]) ;
    };
}

macro_138!()