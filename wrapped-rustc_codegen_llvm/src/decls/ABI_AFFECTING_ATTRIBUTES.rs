macro_rules! deps {
    () => {
        AttributeKind!();
    };
}

macro_rules! ABI_AFFECTING_ATTRIBUTES {
    () => {
        deps!();
        const ABI_AFFECTING_ATTRIBUTES : [(ArgAttribute , llvm :: AttributeKind) ; 1] = [(ArgAttribute :: InReg , llvm :: AttributeKind :: InReg)] ;
    };
}

ABI_AFFECTING_ATTRIBUTES!()