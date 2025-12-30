// Generated macro for impl_238 (impl)
macro_rules! Depcrateimpl_238 {
() => {
// Module: crate
// Provides: {"impl_238"}
// Dependencies: {}
impl std :: fmt :: Debug for MatchDebugInfo { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match & self . matched { Ok (_) => writeln ! (f , "Node matched") ? , Err (reason) => writeln ! (f , "Node failed to match because: {}" , reason . reason) ? , } writeln ! (f , "============ AST ===========\n\
            {:#?}" , self . node) ? ; writeln ! (f , "========= PATTERN ==========") ? ; writeln ! (f , "{:#?}" , self . pattern) ? ; writeln ! (f , "============================") ? ; Ok (()) } }
};
}
