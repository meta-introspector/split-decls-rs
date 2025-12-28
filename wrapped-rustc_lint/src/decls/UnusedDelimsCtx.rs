macro_rules! UnusedDelimsCtx {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum UnusedDelimsCtx { FunctionArg , MethodArg , AssignedValue , AssignedValueLetElse , IfCond , WhileCond , ForIterExpr , MatchScrutineeExpr , ReturnValue , BlockRetValue , BreakValue , LetScrutineeExpr , ArrayLenExpr , AnonConst , MatchArmExpr , IndexExpr , ClosureBody , }
    };
}

UnusedDelimsCtx!()