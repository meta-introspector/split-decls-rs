macro_rules! deps {
    () => {
        Match!();
        MatchFailureReason!();
    };
}

macro_rules! MatchDebugInfo {
    () => {
        deps!();
        pub struct MatchDebugInfo { node : SyntaxNode , # [doc = " Our search pattern parsed as an expression or item, etc"] pattern : SyntaxNode , matched : Result < Match , MatchFailureReason > , }
    };
}

MatchDebugInfo!()