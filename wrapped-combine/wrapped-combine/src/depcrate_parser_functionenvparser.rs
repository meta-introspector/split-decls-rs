// Generated macro for EnvParser (struct)
macro_rules! Depcrate_parser_functionEnvParser {
() => {
// Module: crate::parser::function
// Provides: {"EnvParser"}
// Dependencies: {}
# [derive (Copy)] pub struct EnvParser < E , Input , T > where Input : Stream , { env : E , parser : fn (E , & mut Input) -> StdParseResult < T , Input > , }
};
}
