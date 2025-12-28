macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! reparser {
    () => {
        deps!();
        pub (crate) fn reparser (node : SyntaxKind , first_child : Option < SyntaxKind > , parent : Option < SyntaxKind > ,) -> Option < fn (& mut Parser < '_ >) > { let res = match node { BLOCK_EXPR => expressions :: block_expr , RECORD_FIELD_LIST => items :: record_field_list , RECORD_EXPR_FIELD_LIST => items :: record_expr_field_list , VARIANT_LIST => items :: variant_list , MATCH_ARM_LIST => items :: match_arm_list , USE_TREE_LIST => items :: use_tree_list , EXTERN_ITEM_LIST => items :: extern_item_list , TOKEN_TREE if first_child ? == T ! ['{'] => items :: token_tree , ASSOC_ITEM_LIST => match parent ? { IMPL | TRAIT => items :: assoc_item_list , _ => return None , } , ITEM_LIST => items :: item_list , _ => return None , } ; Some (res) }
    };
}

reparser!()