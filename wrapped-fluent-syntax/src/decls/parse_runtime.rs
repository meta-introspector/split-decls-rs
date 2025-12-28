macro_rules! deps {
    () => {
        Comment!();
        Resource!();
        Parser!();
        Result!();
        Level!();
        Slice!();
    };
}

macro_rules! parse_runtime {
    () => {
        deps!();
        # [doc = " Parses an input into an Abstract Syntax Tree representation with comments stripped."] # [doc = ""] # [doc = " This mode is intended for runtime use of Fluent. It currently strips all"] # [doc = " comments improving parsing performance and reducing the size of the AST tree."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::parser;"] # [doc = " use fluent_syntax::ast;"] # [doc = ""] # [doc = " let ftl = r#\""] # [doc = " #### Resource Level Comment"] # [doc = ""] # [doc = " ## This is a message comment"] # [doc = " hello-world = Hello World!"] # [doc = ""] # [doc = " \"#;"] # [doc = ""] # [doc = " let resource = parser::parse_runtime(ftl)"] # [doc = "     .expect(\"Failed to parse an FTL resource.\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     resource.body[0],"] # [doc = "     ast::Entry::Message("] # [doc = "         ast::Message {"] # [doc = "             id: ast::Identifier {"] # [doc = "                 name: \"hello-world\""] # [doc = "             },"] # [doc = "             value: Some(ast::Pattern {"] # [doc = "                 elements: vec!["] # [doc = "                     ast::PatternElement::TextElement {"] # [doc = "                         value: \"Hello World!\""] # [doc = "                     },"] # [doc = "                 ]"] # [doc = "             }),"] # [doc = "             attributes: vec![],"] # [doc = "             comment: None,"] # [doc = "         }"] # [doc = "     ),"] # [doc = " );"] # [doc = " ```"] pub fn parse_runtime < 's , S > (input : S) -> Result < S > where S : Slice < 's > , { core :: Parser :: new (input) . parse_runtime () }
    };
}

parse_runtime!();