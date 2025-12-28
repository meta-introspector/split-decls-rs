macro_rules! deps {
    () => {
        Op!();
        ParseError!();
        MetaTemplate!();
        MetaVarKind!();
        RepeatKind!();
    };
}

macro_rules! validate {
    () => {
        deps!();
        fn validate (pattern : & MetaTemplate) -> Result < () , ParseError > { for op in pattern . iter () { match op { Op :: Subtree { tokens , .. } => validate (tokens) ? , Op :: Repeat { tokens : subtree , separator , .. } => { let lsh_is_empty_seq = separator . is_none () && subtree . iter () . all (| child_op | { match * child_op { Op :: Var { kind : Some (kind) , .. } => kind == MetaVarKind :: Vis , Op :: Repeat { kind : parser :: RepeatKind :: ZeroOrMore | parser :: RepeatKind :: ZeroOrOne , .. } => true , _ => false , } }) ; if lsh_is_empty_seq { return Err (ParseError :: RepetitionEmptyTokenTree) ; } validate (subtree) ? } _ => () , } } Ok (()) }
    };
}

validate!();