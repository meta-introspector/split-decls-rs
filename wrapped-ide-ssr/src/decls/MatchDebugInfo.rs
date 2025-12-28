macro_rules! MatchDebugInfo {
    () => {
        pub struct MatchDebugInfo { node : SyntaxNode , # [doc = " Our search pattern parsed as an expression or item, etc"] pattern : SyntaxNode , matched : Result < Match , MatchFailureReason > , }
    };
}

MatchDebugInfo!()