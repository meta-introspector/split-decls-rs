macro_rules! deps {
    () => {
        MacroCallId!();
        ExpandError!();
    };
}

macro_rules! EagerCallInfo {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub struct EagerCallInfo { # [doc = " The expanded argument of the eager macro."] arg : Arc < tt :: TopSubtree > , # [doc = " Call id of the eager macro's input file (this is the macro file for its fully expanded input)."] arg_id : MacroCallId , error : Option < ExpandError > , # [doc = " The call site span of the eager macro"] span : Span , }
    };
}

EagerCallInfo!();