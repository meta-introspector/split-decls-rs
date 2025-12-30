// Generated macro for alt_trait_inner (macro)
macro_rules! Depcrate_branchalt_trait_inner {
() => {
// Module: crate::branch
// Provides: {"alt_trait_inner"}
// Dependencies: {}
macro_rules ! alt_trait_inner (($ it : tt , $ self : expr , $ input : expr , $ err : expr , $ head : ident $ ($ id : ident) +) => (match $ self . parser .$ it . process ::< OM > ($ input . clone ()) { Ok (res) => Ok (res) , Err (Err :: Failure (e)) => Err (Err :: Failure (e)) , Err (Err :: Incomplete (i)) => Err (Err :: Incomplete (i)) , Err (Err :: Error (e)) => { succ ! ($ it , alt_trait_inner ! ($ self , $ input , < OM :: Error as crate :: Mode >:: combine ($ err , e , | e1 , e2 | e1 . or (e2)) , $ ($ id) +)) } }) ; ($ it : tt , $ self : expr , $ input : expr , $ err : expr , $ head : ident) => (Err (Err :: Error (< OM :: Error as crate :: Mode >:: map ($ err , | err | Error :: append ($ input , ErrorKind :: Alt , err))))) ;) ;
};
}
