macro_rules! deps {
    () => {
        CfgExpr!();
    };
}

macro_rules! flatten {
    () => {
        deps!();
        # [doc = " Collapses nested `any()` and `all()` predicates."] fn flatten (expr : CfgExpr) -> CfgExpr { match expr { CfgExpr :: All (inner) => CfgExpr :: All (inner . iter () . flat_map (| e | match e { CfgExpr :: All (inner) => inner . as_ref () , _ => std :: slice :: from_ref (e) , }) . cloned () . collect () ,) , CfgExpr :: Any (inner) => CfgExpr :: Any (inner . iter () . flat_map (| e | match e { CfgExpr :: Any (inner) => inner . as_ref () , _ => std :: slice :: from_ref (e) , }) . cloned () . collect () ,) , _ => expr , } }
    };
}

flatten!()