macro_rules! deps {
    () => {
        SingleArmMatchBegin!();
        ConsequentRewrite!();
        ClosingBrackets!();
        AltHead!();
    };
}

macro_rules! IfLetRescopeRewrite {
    () => {
        deps!();
        struct IfLetRescopeRewrite { match_heads : Vec < SingleArmMatchBegin > , consequent_heads : Vec < ConsequentRewrite > , closing_brackets : ClosingBrackets , alt_heads : Vec < AltHead > , }
    };
}

IfLetRescopeRewrite!()