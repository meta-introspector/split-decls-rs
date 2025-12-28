macro_rules! ParamMode {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug)] enum ParamMode { # [doc = " Any path in a type context."] Explicit , # [doc = " The `module::Type` in `module::Type::method` in an expression."] Optional , }
    };
}

ParamMode!();