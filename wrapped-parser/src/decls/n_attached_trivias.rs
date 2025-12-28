macro_rules! n_attached_trivias {
    () => {
        fn n_attached_trivias < 'a > (kind : SyntaxKind , trivias : impl Iterator < Item = (SyntaxKind , & 'a str) > ,) -> usize { match kind { CONST | ENUM | FN | IMPL | MACRO_CALL | MACRO_DEF | MACRO_RULES | MODULE | RECORD_FIELD | STATIC | STRUCT | TRAIT | TUPLE_FIELD | TYPE_ALIAS | UNION | USE | VARIANT | EXTERN_CRATE => { let mut res = 0 ; let mut trivias = trivias . enumerate () . peekable () ; while let Some ((i , (kind , text))) = trivias . next () { match kind { WHITESPACE if text . contains ("\n\n") => { if let Some ((COMMENT , peek_text)) = trivias . peek () . map (| (_ , pair) | pair) && is_outer (peek_text) { continue ; } break ; } COMMENT => { if is_inner (text) { break ; } res = i + 1 ; } _ => () , } } res } _ => 0 , } }
    };
}

n_attached_trivias!();