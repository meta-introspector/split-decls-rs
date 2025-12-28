macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! find_ancestors {
    () => {
        deps!();
        fn find_ancestors (item : SyntaxElement , direction : Direction , range : TextRange) -> Option < TextEdit > { let root = match item { SyntaxElement :: Node (node) => node , SyntaxElement :: Token (token) => token . parent () ? , } ; let movable = [SyntaxKind :: ARG_LIST , SyntaxKind :: GENERIC_PARAM_LIST , SyntaxKind :: GENERIC_ARG_LIST , SyntaxKind :: VARIANT_LIST , SyntaxKind :: TYPE_BOUND_LIST , SyntaxKind :: MATCH_ARM , SyntaxKind :: PARAM , SyntaxKind :: LET_STMT , SyntaxKind :: EXPR_STMT , SyntaxKind :: IF_EXPR , SyntaxKind :: FOR_EXPR , SyntaxKind :: LOOP_EXPR , SyntaxKind :: WHILE_EXPR , SyntaxKind :: RETURN_EXPR , SyntaxKind :: MATCH_EXPR , SyntaxKind :: MACRO_CALL , SyntaxKind :: TYPE_ALIAS , SyntaxKind :: TRAIT , SyntaxKind :: IMPL , SyntaxKind :: MACRO_DEF , SyntaxKind :: STRUCT , SyntaxKind :: UNION , SyntaxKind :: ENUM , SyntaxKind :: FN , SyntaxKind :: MODULE , SyntaxKind :: USE , SyntaxKind :: STATIC , SyntaxKind :: CONST , SyntaxKind :: MACRO_RULES , SyntaxKind :: MACRO_DEF ,] ; let ancestor = once (root . clone ()) . chain (root . ancestors ()) . find (| ancestor | movable . contains (& ancestor . kind ())) ? ; move_in_direction (& ancestor , direction , range) }
    };
}

find_ancestors!();