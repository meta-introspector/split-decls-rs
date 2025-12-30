// Generated macro for internal (module)
macro_rules! Depcrateinternal {
() => {
// Module: crate
// Provides: {"internal"}
// Dependencies: {}
# [doc = " The `internal` module exists to support the `regex!` macro and other"] # [doc = " suspicious activity, such as testing different matching engines and"] # [doc = " supporting the `regex-debug` CLI utility."] # [doc (hidden)] pub mod internal { pub use compile :: Compiler ; pub use exec :: { Exec , ExecBuilder } ; pub use input :: { Char , Input , CharInput , InputAt } ; pub use literals :: LiteralSearcher ; pub use prog :: { Program , Inst , EmptyLook , InstRanges } ; pub use re_plugin :: Plugin ; pub use re_unicode :: _Regex ; }
};
}
