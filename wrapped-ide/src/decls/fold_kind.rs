macro_rules! deps {
    () => {
        FoldKind!();
    };
}

macro_rules! fold_kind {
    () => {
        deps!();
        fn fold_kind (kind : SyntaxKind) -> Option < FoldKind > { match kind { COMMENT => Some (FoldKind :: Comment) , ARG_LIST | PARAM_LIST | GENERIC_ARG_LIST | GENERIC_PARAM_LIST => Some (FoldKind :: ArgList) , ARRAY_EXPR => Some (FoldKind :: Array) , RET_TYPE => Some (FoldKind :: ReturnType) , FN => Some (FoldKind :: Function) , WHERE_CLAUSE => Some (FoldKind :: WhereClause) , ASSOC_ITEM_LIST | RECORD_FIELD_LIST | RECORD_PAT_FIELD_LIST | RECORD_EXPR_FIELD_LIST | ITEM_LIST | EXTERN_ITEM_LIST | USE_TREE_LIST | BLOCK_EXPR | MATCH_ARM_LIST | VARIANT_LIST | TOKEN_TREE => Some (FoldKind :: Block) , _ => None , } }
    };
}

fold_kind!()