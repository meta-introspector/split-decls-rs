macro_rules! deps {
    () => {
        BlockLike!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl BlockLike { fn is_block (self) -> bool { self == BlockLike :: Block } fn is_blocklike (kind : SyntaxKind) -> bool { matches ! (kind , BLOCK_EXPR | IF_EXPR | WHILE_EXPR | FOR_EXPR | LOOP_EXPR | MATCH_EXPR) } }
    };
}

impl_31!();