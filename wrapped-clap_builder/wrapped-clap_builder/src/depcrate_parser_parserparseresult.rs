// Generated macro for ParseResult (enum)
macro_rules! Depcrate_parser_parserParseResult {
() => {
// Module: crate::parser::parser
// Provides: {"ParseResult"}
// Dependencies: {}
# [doc = " Recoverable Parsing results."] # [derive (Debug , PartialEq , Clone)] # [must_use] enum ParseResult { FlagSubCommand (String) , Opt (Id) , ValuesDone , # [doc = " Value attached to the short flag is not consumed(e.g. 'u' for `-cu` is"] # [doc = " not consumed)."] AttachedValueNotConsumed , # [doc = " This long flag doesn't need a value but is provided one."] UnneededAttachedValue { rest : String , used : Vec < Id > , arg : String , } , # [doc = " This flag might be an hyphen Value."] MaybeHyphenValue , # [doc = " Equals required but not provided."] EqualsNotProvided { arg : String , } , # [doc = " Failed to match a Arg."] NoMatchingArg { arg : String , } , # [doc = " No argument found e.g. parser is given `-` when parsing a flag."] NoArg , }
};
}
