macro_rules! BreakOutsideOfLoop {
    () => {
        # [derive (Debug)] pub struct BreakOutsideOfLoop { pub expr : InFile < ExprOrPatPtr > , pub is_break : bool , pub bad_value_break : bool , }
    };
}

BreakOutsideOfLoop!();