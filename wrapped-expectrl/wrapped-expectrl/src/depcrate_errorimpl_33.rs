// Generated macro for impl_33 (impl)
macro_rules! Depcrate_errorimpl_33 {
() => {
// Module: crate::error
// Provides: {"impl_33"}
// Dependencies: {}
impl Display for Error { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Error :: IO (err) => write ! (f , "IO error {}" , err) , Error :: CommandParsing => write ! (f , "Can't parse a command string, please check it out") , Error :: RegexParsing => write ! (f , "Can't parse a regex expression") , Error :: ExpectTimeout => write ! (f , "Reached a timeout for expect type of command") , Error :: Eof => write ! (f , "EOF was reached; the read may successed later") , Error :: Other { message , err } => write ! (f , "Unexpected error; {}; {}" , message , err) , } } }
};
}
