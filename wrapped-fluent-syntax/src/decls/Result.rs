macro_rules! deps {
    () => {
        Parser!();
        ParserError!();
        Resource!();
    };
}

macro_rules! Result {
    () => {
        deps!();
        # [doc = " Parser result always returns an AST representation of the input,"] # [doc = " and if parsing errors were encountered, a list of [`ParserError`] elements"] # [doc = " is also returned."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::parser;"] # [doc = " use fluent_syntax::ast;"] # [doc = ""] # [doc = " let ftl = r#\""] # [doc = " key1 = Value 1"] # [doc = ""] # [doc = " g@Rb@ge = #2y ds"] # [doc = ""] # [doc = " key2 = Value 2"] # [doc = ""] # [doc = " \"#;"] # [doc = ""] # [doc = " let (resource, errors) = parser::parse_runtime(ftl)"] # [doc = "     .expect_err(\"Resource should contain errors.\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     errors,"] # [doc = "     vec!["] # [doc = "         parser::ParserError {"] # [doc = "             pos: 18..19,"] # [doc = "             slice: Some(17..35),"] # [doc = "             kind: parser::ErrorKind::ExpectedToken('=')"] # [doc = "         }"] # [doc = "     ]"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     resource.body[0],"] # [doc = "     ast::Entry::Message("] # [doc = "         ast::Message {"] # [doc = "             id: ast::Identifier {"] # [doc = "                 name: \"key1\""] # [doc = "             },"] # [doc = "             value: Some(ast::Pattern {"] # [doc = "                 elements: vec!["] # [doc = "                     ast::PatternElement::TextElement {"] # [doc = "                         value: \"Value 1\""] # [doc = "                     },"] # [doc = "                 ]"] # [doc = "             }),"] # [doc = "             attributes: vec![],"] # [doc = "             comment: None,"] # [doc = "         }"] # [doc = "     ),"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     resource.body[1],"] # [doc = "     ast::Entry::Junk {"] # [doc = "         content: \"g@Rb@ge = #2y ds\\n\\n\""] # [doc = "     }"] # [doc = " );"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     resource.body[2],"] # [doc = "     ast::Entry::Message("] # [doc = "         ast::Message {"] # [doc = "             id: ast::Identifier {"] # [doc = "                 name: \"key2\""] # [doc = "             },"] # [doc = "             value: Some(ast::Pattern {"] # [doc = "                 elements: vec!["] # [doc = "                     ast::PatternElement::TextElement {"] # [doc = "                         value: \"Value 2\""] # [doc = "                     },"] # [doc = "                 ]"] # [doc = "             }),"] # [doc = "             attributes: vec![],"] # [doc = "             comment: None,"] # [doc = "         }"] # [doc = "     ),"] # [doc = " );"] # [doc = " ```"] pub type Result < S > = std :: result :: Result < ast :: Resource < S > , (ast :: Resource < S > , Vec < ParserError >) > ;
    };
}

Result!();