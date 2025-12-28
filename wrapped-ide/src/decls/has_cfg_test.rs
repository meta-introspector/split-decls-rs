macro_rules! has_cfg_test {
    () => {
        fn has_cfg_test (attrs : AttrsWithOwner) -> bool { attrs . cfgs () . any (| cfg | matches ! (& cfg , CfgExpr :: Atom (CfgAtom :: Flag (s)) if * s == sym :: test)) }
    };
}

has_cfg_test!();