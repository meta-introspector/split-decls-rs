macro_rules! deps {
    () => {
        ConcatMetaVarExprElem!();
        MetaVarKind!();
        RepeatKind!();
        MetaTemplate!();
        Separator!();
    };
}

macro_rules! Op {
    () => {
        deps!();
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum Op { Var { name : Symbol , kind : Option < MetaVarKind > , id : Span , } , Ignore { name : Symbol , id : Span , } , Index { depth : usize , } , Len { depth : usize , } , Count { name : Symbol , depth : Option < usize > , } , Concat { elements : Box < [ConcatMetaVarExprElem] > , span : Span , } , Repeat { tokens : MetaTemplate , kind : RepeatKind , separator : Option < Arc < Separator > > , } , Subtree { tokens : MetaTemplate , delimiter : tt :: Delimiter < Span > , } , Literal (tt :: Literal < Span >) , Punct (Box < ArrayVec < tt :: Punct < Span > , MAX_GLUED_PUNCT_LEN > >) , Ident (tt :: Ident < Span >) , }
    };
}

Op!();