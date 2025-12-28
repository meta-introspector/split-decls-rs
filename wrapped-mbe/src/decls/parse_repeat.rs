macro_rules! deps {
    () => {
        ParseError!();
        Separator!();
        RepeatKind!();
    };
}

macro_rules! parse_repeat {
    () => {
        deps!();
        fn parse_repeat (src : & mut TtIter < '_ , Span >) -> Result < (Option < Separator > , RepeatKind) , ParseError > { let mut separator = Separator :: Puncts (ArrayVec :: new ()) ; for tt in src { let tt = match tt { TtElement :: Leaf (leaf) => leaf , TtElement :: Subtree (..) => return Err (ParseError :: InvalidRepeat) , } ; let has_sep = match & separator { Separator :: Puncts (puncts) => ! puncts . is_empty () , _ => true , } ; match tt { tt :: Leaf :: Ident (ident) => match separator { Separator :: Puncts (puncts) if puncts . is_empty () => { separator = Separator :: Ident (ident . clone ()) ; } Separator :: Puncts (puncts) => match puncts . as_slice () { [tt :: Punct { char : '\'' , .. }] => { separator = Separator :: Lifetime (puncts [0] , ident . clone ()) ; } _ => return Err (ParseError :: InvalidRepeat) , } , _ => return Err (ParseError :: InvalidRepeat) , } , tt :: Leaf :: Literal (_) if has_sep => return Err (ParseError :: InvalidRepeat) , tt :: Leaf :: Literal (lit) => separator = Separator :: Literal (lit . clone ()) , tt :: Leaf :: Punct (punct) => { let repeat_kind = match punct . char { '*' => RepeatKind :: ZeroOrMore , '+' => RepeatKind :: OneOrMore , '?' => RepeatKind :: ZeroOrOne , _ => match & mut separator { Separator :: Puncts (puncts) if puncts . len () < 3 => { puncts . push (* punct) ; continue ; } _ => return Err (ParseError :: InvalidRepeat) , } , } ; return Ok ((has_sep . then_some (separator) , repeat_kind)) ; } } } Err (ParseError :: InvalidRepeat) }
    };
}

parse_repeat!()