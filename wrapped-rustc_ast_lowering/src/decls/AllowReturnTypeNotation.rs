macro_rules! AllowReturnTypeNotation {
    () => {
        # [derive (Copy , Clone , Debug)] enum AllowReturnTypeNotation { # [doc = " Only in types, since RTN is denied later during HIR lowering."] Yes , # [doc = " All other positions (path expr, method, use tree)."] No , }
    };
}

AllowReturnTypeNotation!();