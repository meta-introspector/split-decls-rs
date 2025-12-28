macro_rules! macro_782 {
    () => {
        declare_lint_pass ! (# [doc = " Lint for bounds of the form `T: Drop`, which usually"] # [doc = " indicate an attempt to emulate `std::mem::needs_drop`."] DropTraitConstraints => [DROP_BOUNDS , DYN_DROP]) ;
    };
}

macro_782!()