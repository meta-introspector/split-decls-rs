macro_rules! deps {
    () => {
        HygieneId!();
        ExprId!();
        ExprPtr!();
    };
}

macro_rules! FormatTemplate {
    () => {
        deps!();
        # [derive (Default , Debug , Eq , PartialEq)] struct FormatTemplate { # [doc = " A map from `format_args!()` expressions to their captures."] format_args_to_captures : FxHashMap < ExprId , (HygieneId , Vec < (syntax :: TextRange , Name) >) > , # [doc = " A map from `asm!()` expressions to their captures."] asm_to_captures : FxHashMap < ExprId , Vec < Vec < (syntax :: TextRange , usize) > > > , # [doc = " A map from desugared expressions of implicit captures to their source."] # [doc = ""] # [doc = " The value stored for each capture is its template literal and offset inside it. The template literal"] # [doc = " is from the `format_args[_nl]!()` macro and so needs to be mapped up once to go to the user-written"] # [doc = " template."] implicit_capture_to_source : FxHashMap < ExprId , InFile < (ExprPtr , TextRange) > > , }
    };
}

FormatTemplate!()