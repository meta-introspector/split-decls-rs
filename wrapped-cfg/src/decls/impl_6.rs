macro_rules! deps {
    () => {
        CfgAtom!();
        CfgExpr!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl CfgExpr { # [cfg (feature = "tt")] pub fn parse < S : Copy > (tt : & tt :: TopSubtree < S >) -> CfgExpr { next_cfg_expr (& mut tt . iter ()) . unwrap_or (CfgExpr :: Invalid) } # [cfg (feature = "tt")] pub fn parse_from_iter < S : Copy > (tt : & mut tt :: iter :: TtIter < '_ , S >) -> CfgExpr { next_cfg_expr (tt) . unwrap_or (CfgExpr :: Invalid) } # [doc = " Fold the cfg by querying all basic `Atom` and `KeyValue` predicates."] pub fn fold (& self , query : & dyn Fn (& CfgAtom) -> bool) -> Option < bool > { match self { CfgExpr :: Invalid => None , CfgExpr :: Atom (atom) => Some (query (atom)) , CfgExpr :: All (preds) => { preds . iter () . try_fold (true , | s , pred | Some (s && pred . fold (query) ?)) } CfgExpr :: Any (preds) => { preds . iter () . try_fold (false , | s , pred | Some (s || pred . fold (query) ?)) } CfgExpr :: Not (pred) => pred . fold (query) . map (| s | ! s) , } } }
    };
}

impl_6!();