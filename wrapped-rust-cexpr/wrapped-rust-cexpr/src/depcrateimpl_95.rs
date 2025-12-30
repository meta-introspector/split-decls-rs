// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl < I , O , E > ToCexprResult < I , O > for nom :: IResult < I , O , E > where Error < I > : From < E > , { fn to_cexpr_result (self) -> nom :: IResult < I , O , Error < I > > { match self { Ok (v) => Ok (v) , Err (nom :: Err :: Incomplete (n)) => Err (nom :: Err :: Incomplete (n)) , Err (nom :: Err :: Error (e)) => Err (nom :: Err :: Error (e . into ())) , Err (nom :: Err :: Failure (e)) => Err (nom :: Err :: Failure (e . into ())) , } } }
};
}
