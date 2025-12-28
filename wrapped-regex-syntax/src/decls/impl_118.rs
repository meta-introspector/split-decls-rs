macro_rules! deps {
    () => {
        Concat!();
        Position!();
        Assertion!();
        Literal!();
        Repetition!();
        Alternation!();
        ClassPerl!();
        Flags!();
        Dot!();
        Span!();
        ClassBracketed!();
        Group!();
        Ast!();
        ClassUnicode!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        # [doc = " A custom `Drop` impl is used for `Ast` such that it uses constant stack"] # [doc = " space but heap space proportional to the depth of the `Ast`."] impl Drop for Ast { fn drop (& mut self) { use core :: mem ; match * self { Ast :: Empty (_) | Ast :: Flags (_) | Ast :: Literal (_) | Ast :: Dot (_) | Ast :: Assertion (_) | Ast :: ClassUnicode (_) | Ast :: ClassPerl (_) | Ast :: ClassBracketed (_) => return , Ast :: Repetition (ref x) if ! x . ast . has_subexprs () => return , Ast :: Group (ref x) if ! x . ast . has_subexprs () => return , Ast :: Alternation (ref x) if x . asts . is_empty () => return , Ast :: Concat (ref x) if x . asts . is_empty () => return , _ => { } } let empty_span = | | Span :: splat (Position :: new (0 , 0 , 0)) ; let empty_ast = | | Ast :: empty (empty_span ()) ; let mut stack = vec ! [mem :: replace (self , empty_ast ())] ; while let Some (mut ast) = stack . pop () { match ast { Ast :: Empty (_) | Ast :: Flags (_) | Ast :: Literal (_) | Ast :: Dot (_) | Ast :: Assertion (_) | Ast :: ClassUnicode (_) | Ast :: ClassPerl (_) | Ast :: ClassBracketed (_) => { } Ast :: Repetition (ref mut x) => { stack . push (mem :: replace (& mut x . ast , empty_ast ())) ; } Ast :: Group (ref mut x) => { stack . push (mem :: replace (& mut x . ast , empty_ast ())) ; } Ast :: Alternation (ref mut x) => { stack . extend (x . asts . drain (..)) ; } Ast :: Concat (ref mut x) => { stack . extend (x . asts . drain (..)) ; } } } } }
    };
}

impl_118!()