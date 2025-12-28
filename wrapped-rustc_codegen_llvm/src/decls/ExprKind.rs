macro_rules! ExprKind {
    () => {
        # [doc = " Corresponds to enum `llvm::coverage::CounterExpression::ExprKind`."] # [doc = ""] # [doc = " Must match the layout of `LLVMRustCounterExprKind`."] # [derive (Copy , Clone , Debug)] # [repr (C)] pub (crate) enum ExprKind { Subtract = 0 , Add = 1 , }
    };
}

ExprKind!();