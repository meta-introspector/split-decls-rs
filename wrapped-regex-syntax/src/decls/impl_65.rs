macro_rules! deps {
    () => {
        Assertion!();
        SetFlags!();
        Group!();
        Ast!();
        Dot!();
        ClassBracketed!();
        Alternation!();
        Concat!();
        Repetition!();
        Literal!();
        ClassPerl!();
        ClassUnicode!();
        Flags!();
        Span!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl Ast { # [doc = " Create an \"empty\" AST item."] pub fn empty (span : Span) -> Ast { Ast :: Empty (Box :: new (span)) } # [doc = " Create a \"flags\" AST item."] pub fn flags (e : SetFlags) -> Ast { Ast :: Flags (Box :: new (e)) } # [doc = " Create a \"literal\" AST item."] pub fn literal (e : Literal) -> Ast { Ast :: Literal (Box :: new (e)) } # [doc = " Create a \"dot\" AST item."] pub fn dot (span : Span) -> Ast { Ast :: Dot (Box :: new (span)) } # [doc = " Create a \"assertion\" AST item."] pub fn assertion (e : Assertion) -> Ast { Ast :: Assertion (Box :: new (e)) } # [doc = " Create a \"Unicode class\" AST item."] pub fn class_unicode (e : ClassUnicode) -> Ast { Ast :: ClassUnicode (Box :: new (e)) } # [doc = " Create a \"Perl class\" AST item."] pub fn class_perl (e : ClassPerl) -> Ast { Ast :: ClassPerl (Box :: new (e)) } # [doc = " Create a \"bracketed class\" AST item."] pub fn class_bracketed (e : ClassBracketed) -> Ast { Ast :: ClassBracketed (Box :: new (e)) } # [doc = " Create a \"repetition\" AST item."] pub fn repetition (e : Repetition) -> Ast { Ast :: Repetition (Box :: new (e)) } # [doc = " Create a \"group\" AST item."] pub fn group (e : Group) -> Ast { Ast :: Group (Box :: new (e)) } # [doc = " Create a \"alternation\" AST item."] pub fn alternation (e : Alternation) -> Ast { Ast :: Alternation (Box :: new (e)) } # [doc = " Create a \"concat\" AST item."] pub fn concat (e : Concat) -> Ast { Ast :: Concat (Box :: new (e)) } # [doc = " Return the span of this abstract syntax tree."] pub fn span (& self) -> & Span { match * self { Ast :: Empty (ref span) => span , Ast :: Flags (ref x) => & x . span , Ast :: Literal (ref x) => & x . span , Ast :: Dot (ref span) => span , Ast :: Assertion (ref x) => & x . span , Ast :: ClassUnicode (ref x) => & x . span , Ast :: ClassPerl (ref x) => & x . span , Ast :: ClassBracketed (ref x) => & x . span , Ast :: Repetition (ref x) => & x . span , Ast :: Group (ref x) => & x . span , Ast :: Alternation (ref x) => & x . span , Ast :: Concat (ref x) => & x . span , } } # [doc = " Return true if and only if this Ast is empty."] pub fn is_empty (& self) -> bool { match * self { Ast :: Empty (_) => true , _ => false , } } # [doc = " Returns true if and only if this AST has any (including possibly empty)"] # [doc = " subexpressions."] fn has_subexprs (& self) -> bool { match * self { Ast :: Empty (_) | Ast :: Flags (_) | Ast :: Literal (_) | Ast :: Dot (_) | Ast :: Assertion (_) | Ast :: ClassUnicode (_) | Ast :: ClassPerl (_) => false , Ast :: ClassBracketed (_) | Ast :: Repetition (_) | Ast :: Group (_) | Ast :: Alternation (_) | Ast :: Concat (_) => true , } } }
    };
}

impl_65!()