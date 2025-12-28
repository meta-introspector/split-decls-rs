macro_rules! deps {
    () => {
        Register!();
        Expression!();
    };
}

macro_rules! CallFrameInstruction {
    () => {
        deps!();
        # [doc = " An instruction in a frame description entry."] # [doc = ""] # [doc = " This may be a CFA definition, a register rule, or some other directive."] # [derive (Debug , Clone , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum CallFrameInstruction { # [doc = " Define the CFA rule to use the provided register and offset."] Cfa (Register , i32) , # [doc = " Update the CFA rule to use the provided register. The offset is unchanged."] CfaRegister (Register) , # [doc = " Update the CFA rule to use the provided offset. The register is unchanged."] CfaOffset (i32) , # [doc = " Define the CFA rule to use the provided expression."] CfaExpression (Expression) , # [doc = " Restore the initial rule for the register."] Restore (Register) , # [doc = " The previous value of the register is not recoverable."] Undefined (Register) , # [doc = " The register has not been modified."] SameValue (Register) , # [doc = " The previous value of the register is saved at address CFA + offset."] Offset (Register , i32) , # [doc = " The previous value of the register is CFA + offset."] ValOffset (Register , i32) , # [doc = " The previous value of the register is stored in another register."] Register (Register , Register) , # [doc = " The previous value of the register is saved at address given by the expression."] Expression (Register , Expression) , # [doc = " The previous value of the register is given by the expression."] ValExpression (Register , Expression) , # [doc = " Push all register rules onto a stack."] RememberState , # [doc = " Pop all register rules off the stack."] RestoreState , # [doc = " The size of the arguments that have been pushed onto the stack."] ArgsSize (u32) , # [doc = " AAarch64 extension: negate the `RA_SIGN_STATE` pseudo-register."] NegateRaState , }
    };
}

CallFrameInstruction!()