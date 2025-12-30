// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn context_test () { use crate :: { character :: char , combinator :: cut , internal :: Needed } ; # [derive (Debug , PartialEq)] struct Error < I > { input : I , ctx : Option < & 'static str > , } impl < I > ParseError < I > for Error < I > { fn from_error_kind (input : I , _kind : ErrorKind) -> Self { Self { input , ctx : None } } fn append (input : I , _kind : ErrorKind , other : Self) -> Self { Self { input , ctx : other . ctx , } } } impl < I > ContextError < I > for Error < I > { fn add_context (input : I , ctx : & 'static str , _other : Self) -> Self { Self { input , ctx : Some (ctx) , } } } assert_eq ! (context ("ctx" , char ::< _ , Error < _ >> ('a')) . parse ("abcd") , Ok (("bcd" , 'a'))) ; assert_eq ! (context ("ctx" , char ::< _ , Error < _ >> ('a')) . parse ("") , Err (Err :: Incomplete (Needed :: new (1)))) ; assert_eq ! (context ("ctx" , char ::< _ , Error < _ >> ('a')) . parse_complete ("") , Err (Err :: Error (Error { input : "" , ctx : Some ("ctx") }))) ; assert_eq ! (context ("ctx" , cut (char ::< _ , Error < _ >> ('a'))) . parse ("bcd") , Err (Err :: Failure (Error { input : "bcd" , ctx : Some ("ctx") }))) ; } # [cfg (feature = "alloc")] # [test] fn clone_error () { use crate :: lib :: std :: string :: String ; let err = Error { code : ErrorKind :: Eof , input : "test" , } ; let _err : Error < String > = err . cloned () ; } # [test] fn copy_error () { let err = Error { code : ErrorKind :: Eof , input : & 0_u8 , } ; let _err : Error < u8 > = err . copied () ; } }
};
}
