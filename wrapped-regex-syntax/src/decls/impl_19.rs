macro_rules! deps {
    () => {
        ClassUnicode!();
        ClassBracketed!();
        Repetition!();
        Dot!();
        Group!();
        ClassSetBinaryOp!();
        ClassPerl!();
        Assertion!();
        Ast!();
        Literal!();
        Flags!();
        NestLimiter!();
        Parser!();
        Range!();
        Concat!();
        Error!();
        Visitor!();
        ClassSetItem!();
        Result!();
        Alternation!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < 'p , 's , P : Borrow < Parser > > ast :: Visitor for NestLimiter < 'p , 's , P > { type Output = () ; type Err = ast :: Error ; fn finish (self) -> Result < () > { Ok (()) } fn visit_pre (& mut self , ast : & Ast) -> Result < () > { let span = match * ast { Ast :: Empty (_) | Ast :: Flags (_) | Ast :: Literal (_) | Ast :: Dot (_) | Ast :: Assertion (_) | Ast :: ClassUnicode (_) | Ast :: ClassPerl (_) => { return Ok (()) ; } Ast :: ClassBracketed (ref x) => & x . span , Ast :: Repetition (ref x) => & x . span , Ast :: Group (ref x) => & x . span , Ast :: Alternation (ref x) => & x . span , Ast :: Concat (ref x) => & x . span , } ; self . increment_depth (span) } fn visit_post (& mut self , ast : & Ast) -> Result < () > { match * ast { Ast :: Empty (_) | Ast :: Flags (_) | Ast :: Literal (_) | Ast :: Dot (_) | Ast :: Assertion (_) | Ast :: ClassUnicode (_) | Ast :: ClassPerl (_) => { Ok (()) } Ast :: ClassBracketed (_) | Ast :: Repetition (_) | Ast :: Group (_) | Ast :: Alternation (_) | Ast :: Concat (_) => { self . decrement_depth () ; Ok (()) } } } fn visit_class_set_item_pre (& mut self , ast : & ast :: ClassSetItem ,) -> Result < () > { let span = match * ast { ast :: ClassSetItem :: Empty (_) | ast :: ClassSetItem :: Literal (_) | ast :: ClassSetItem :: Range (_) | ast :: ClassSetItem :: Ascii (_) | ast :: ClassSetItem :: Unicode (_) | ast :: ClassSetItem :: Perl (_) => { return Ok (()) ; } ast :: ClassSetItem :: Bracketed (ref x) => & x . span , ast :: ClassSetItem :: Union (ref x) => & x . span , } ; self . increment_depth (span) } fn visit_class_set_item_post (& mut self , ast : & ast :: ClassSetItem ,) -> Result < () > { match * ast { ast :: ClassSetItem :: Empty (_) | ast :: ClassSetItem :: Literal (_) | ast :: ClassSetItem :: Range (_) | ast :: ClassSetItem :: Ascii (_) | ast :: ClassSetItem :: Unicode (_) | ast :: ClassSetItem :: Perl (_) => { Ok (()) } ast :: ClassSetItem :: Bracketed (_) | ast :: ClassSetItem :: Union (_) => { self . decrement_depth () ; Ok (()) } } } fn visit_class_set_binary_op_pre (& mut self , ast : & ast :: ClassSetBinaryOp ,) -> Result < () > { self . increment_depth (& ast . span) } fn visit_class_set_binary_op_post (& mut self , _ast : & ast :: ClassSetBinaryOp ,) -> Result < () > { self . decrement_depth () ; Ok (()) } }
    };
}

impl_19!()