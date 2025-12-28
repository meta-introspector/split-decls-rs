macro_rules! deps {
    () => {
        Pattern!();
        Identifier!();
        Message!();
        Attribute!();
        Comment!();
    };
}

macro_rules! Term {
    () => {
        deps!();
        # [doc = " A Fluent [`Term`]."] # [doc = ""] # [doc = " Terms are semantically similar to [`Message`] nodes, but"] # [doc = " they represent a separate concept in Fluent system."] # [doc = ""] # [doc = " Every term has to have a value, and the parser will"] # [doc = " report errors when term references are used in wrong positions."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::parser;"] # [doc = " use fluent_syntax::ast;"] # [doc = ""] # [doc = " let ftl = r#\""] # [doc = ""] # [doc = " -brand-name = Nightly"] # [doc = ""] # [doc = " \"#;"] # [doc = ""] # [doc = " let resource = parser::parse(ftl)"] # [doc = "     .expect(\"Failed to parse an FTL resource.\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     resource,"] # [doc = "     ast::Resource {"] # [doc = "         body: vec!["] # [doc = "             ast::Entry::Term(ast::Term {"] # [doc = "                 id: ast::Identifier {"] # [doc = "                     name: \"brand-name\""] # [doc = "                 },"] # [doc = "                 value: ast::Pattern {"] # [doc = "                     elements: vec!["] # [doc = "                         ast::PatternElement::TextElement {"] # [doc = "                             value: \"Nightly\""] # [doc = "                         }"] # [doc = "                     ]"] # [doc = "                 },"] # [doc = "                 attributes: vec![],"] # [doc = "                 comment: None,"] # [doc = "             })"] # [doc = "         ]"] # [doc = "     }"] # [doc = " );"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Term < S > { pub id : Identifier < S > , pub value : Pattern < S > , pub attributes : Vec < Attribute < S > > , pub comment : Option < Comment < S > > , }
    };
}

Term!()