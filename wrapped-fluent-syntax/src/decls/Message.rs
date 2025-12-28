macro_rules! deps {
    () => {
        Identifier!();
        Pattern!();
        Entry!();
        Attribute!();
        Comment!();
        Resource!();
    };
}

macro_rules! Message {
    () => {
        deps!();
        # [doc = " Message node represents the most common [`Entry`] in an FTL [`Resource`]."] # [doc = ""] # [doc = " A message is a localization unit with a [`Identifier`] unique within a given"] # [doc = " [`Resource`], and a value or attributes with associated [`Pattern`]."] # [doc = ""] # [doc = " A message can contain a simple text value, or a compound combination of value"] # [doc = " and attributes which together can be used to localize a complex User Interface"] # [doc = " element."] # [doc = ""] # [doc = " Finally, each [`Message`] may have an associated [`Comment`]."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::parser;"] # [doc = " use fluent_syntax::ast;"] # [doc = ""] # [doc = " let ftl = r#\""] # [doc = ""] # [doc = " hello-world = Hello, World!"] # [doc = ""] # [doc = " \"#;"] # [doc = ""] # [doc = " let resource = parser::parse(ftl)"] # [doc = "     .expect(\"Failed to parse an FTL resource.\");"] # [doc = ""] # [doc = " assert_eq!("] # [doc = "     resource,"] # [doc = "     ast::Resource {"] # [doc = "         body: vec!["] # [doc = "             ast::Entry::Message(ast::Message {"] # [doc = "                 id: ast::Identifier {"] # [doc = "                     name: \"hello-world\""] # [doc = "                 },"] # [doc = "                 value: Some(ast::Pattern {"] # [doc = "                     elements: vec!["] # [doc = "                         ast::PatternElement::TextElement {"] # [doc = "                             value: \"Hello, World!\""] # [doc = "                         }"] # [doc = "                     ]"] # [doc = "                 }),"] # [doc = "                 attributes: vec![],"] # [doc = "                 comment: None,"] # [doc = "             })"] # [doc = "         ]"] # [doc = "     }"] # [doc = " );"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] # [cfg_attr (feature = "serde" , derive (Serialize , Deserialize))] pub struct Message < S > { pub id : Identifier < S > , pub value : Option < Pattern < S > > , pub attributes : Vec < Attribute < S > > , pub comment : Option < Comment < S > > , }
    };
}

Message!()