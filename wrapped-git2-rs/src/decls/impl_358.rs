macro_rules! deps {
    () => {
        DiffBinaryKind!();
        Delta!();
        Binding!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl Binding for DiffBinaryKind { type Raw = raw :: git_diff_binary_t ; unsafe fn from_raw (raw : raw :: git_diff_binary_t) -> DiffBinaryKind { match raw { raw :: GIT_DIFF_BINARY_NONE => DiffBinaryKind :: None , raw :: GIT_DIFF_BINARY_LITERAL => DiffBinaryKind :: Literal , raw :: GIT_DIFF_BINARY_DELTA => DiffBinaryKind :: Delta , _ => panic ! ("Unknown git diff binary kind") , } } fn raw (& self) -> raw :: git_diff_binary_t { match * self { DiffBinaryKind :: None => raw :: GIT_DIFF_BINARY_NONE , DiffBinaryKind :: Literal => raw :: GIT_DIFF_BINARY_LITERAL , DiffBinaryKind :: Delta => raw :: GIT_DIFF_BINARY_DELTA , } } }
    };
}

impl_358!();