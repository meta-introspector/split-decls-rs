macro_rules! parse_cfg_attr_input {
    () => {
        fn parse_cfg_attr_input (subtree : & TopSubtree ,) -> Option < (tt :: TokenTreesView < '_ > , impl Iterator < Item = tt :: TokenTreesView < '_ > >) > { let mut parts = subtree . token_trees () . split (| tt | matches ! (tt , tt :: TtElement :: Leaf (tt :: Leaf :: Punct (Punct { char : ',' , .. })))) ; let cfg = parts . next () ? ; Some ((cfg , parts . filter (| it | ! it . is_empty ()))) }
    };
}

parse_cfg_attr_input!();