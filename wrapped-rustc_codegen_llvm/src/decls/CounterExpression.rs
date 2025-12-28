macro_rules! deps {
    () => {
        ExprKind!();
        Counter!();
    };
}

macro_rules! CounterExpression {
    () => {
        deps!();
        # [doc = " Corresponds to struct `llvm::coverage::CounterExpression`."] # [doc = ""] # [doc = " Must match the layout of `LLVMRustCounterExpression`."] # [derive (Copy , Clone , Debug)] # [repr (C)] pub (crate) struct CounterExpression { pub (crate) kind : ExprKind , pub (crate) lhs : Counter , pub (crate) rhs : Counter , }
    };
}

CounterExpression!()