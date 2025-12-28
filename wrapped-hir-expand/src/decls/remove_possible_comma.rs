macro_rules! remove_possible_comma {
    () => {
        # [doc = " Removes a possible comma after the [AstNode]"] fn remove_possible_comma (item : & impl AstNode , res : & mut FxHashSet < SyntaxElement >) { if let Some (comma) = item . syntax () . next_sibling_or_token () . filter (| it | it . kind () == T ! [,]) { res . insert (comma) ; } }
    };
}

remove_possible_comma!();