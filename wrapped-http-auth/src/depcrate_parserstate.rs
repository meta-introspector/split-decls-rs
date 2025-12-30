// Generated macro for State (enum)
macro_rules! Depcrate_parserState {
() => {
// Module: crate::parser
// Provides: {"State"}
// Dependencies: {}
enum State < 'i > { Done , # [doc = " Consuming OWS and commas, then advancing to `Token`."] PreToken { challenge : Option < ChallengeRef < 'i > > , next : Possibilities , } , # [doc = " Parsing a scheme/parameter key, or the whitespace immediately following it."] Token { # [doc = " Current `challenge`, if any. If none, this token must be a scheme."] challenge : Option < ChallengeRef < 'i > > , token_pos : Range < usize > , cur : Possibilities , } , # [doc = " Transitioned from `Token` or `PostToken` on first `=` after parameter key."] # [doc = " Kept there for BWS in param case."] PostEquals { challenge : ChallengeRef < 'i > , key_pos : Range < usize > , } , # [doc = " Transitioned from `Equals` on initial `C_TCHAR`."] ParamUnquotedValue { challenge : ChallengeRef < 'i > , key_pos : Range < usize > , value_start : usize , } , # [doc = " Transitioned from `Equals` on initial `\"`."] ParamQuotedValue { challenge : ChallengeRef < 'i > , key_pos : Range < usize > , value_start : usize , escapes : usize , in_backslash : bool , } , }
};
}
