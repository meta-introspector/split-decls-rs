macro_rules! deps {
    () => {
        AttributeKind!();
    };
}

macro_rules! OPTIMIZATION_ATTRIBUTES {
    () => {
        deps!();
        const OPTIMIZATION_ATTRIBUTES : [(ArgAttribute , llvm :: AttributeKind) ; 6] = [(ArgAttribute :: NoAlias , llvm :: AttributeKind :: NoAlias) , (ArgAttribute :: CapturesAddress , llvm :: AttributeKind :: CapturesAddress) , (ArgAttribute :: NonNull , llvm :: AttributeKind :: NonNull) , (ArgAttribute :: ReadOnly , llvm :: AttributeKind :: ReadOnly) , (ArgAttribute :: NoUndef , llvm :: AttributeKind :: NoUndef) , (ArgAttribute :: CapturesReadOnly , llvm :: AttributeKind :: CapturesReadOnly) ,] ;
    };
}

OPTIMIZATION_ATTRIBUTES!()