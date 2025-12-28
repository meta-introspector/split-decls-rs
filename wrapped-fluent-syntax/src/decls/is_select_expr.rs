macro_rules! deps {
    () => {
        Slice!();
        InlineExpression!();
        Expression!();
    };
}

macro_rules! is_select_expr {
    () => {
        deps!();
        fn is_select_expr < 's , S : Slice < 's > > (expr : & Expression < S >) -> bool { match expr { Expression :: Select { .. } => true , Expression :: Inline (InlineExpression :: Placeable { expression }) => { is_select_expr (expression) } Expression :: Inline (_) => false , } }
    };
}

is_select_expr!()