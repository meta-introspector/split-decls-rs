// Generated macro for assert_full_parse (function)
macro_rules! Depcrateassert_full_parse {
() => {
// Module: crate
// Provides: {"assert_full_parse"}
// Dependencies: {}
# [doc = " If the input result indicates a succesful parse, but there is data left,"] # [doc = " return an `Error::Partial` instead."] pub fn assert_full_parse < 'i , I : 'i , O , E > (result : nom :: IResult < & 'i [I] , O , E > ,) -> nom :: IResult < & 'i [I] , O , Error < & 'i [I] > > where Error < & 'i [I] > : From < E > , { match result . to_cexpr_result () { Ok ((rem , output)) => { if rem . is_empty () { Ok ((rem , output)) } else { Err (nom :: Err :: Error ((rem , ErrorKind :: Partial) . into ())) } } Err (nom :: Err :: Incomplete (n)) => Err (nom :: Err :: Incomplete (n)) , Err (nom :: Err :: Failure (e)) => Err (nom :: Err :: Failure (e)) , Err (nom :: Err :: Error (e)) => Err (nom :: Err :: Error (e)) , } }
};
}
