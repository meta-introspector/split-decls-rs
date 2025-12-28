macro_rules! deps {
    () => {
        Slice!();
        Resource!();
        Options!();
    };
}

macro_rules! serialize {
    () => {
        deps!();
        # [doc = " Serializes an abstract syntax tree representing a Fluent Translation List into a"] # [doc = " String."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use fluent_syntax::parser;"] # [doc = " use fluent_syntax::serializer;"] # [doc = ""] # [doc = " let ftl = r#\""] # [doc = " unnormalized-message=This message has"] # [doc = "   abnormal spacing and indentation\"#;"] # [doc = ""] # [doc = " let resource = parser::parse(ftl).expect(\"Failed to parse an FTL resource.\");"] # [doc = ""] # [doc = " let serialized = serializer::serialize(&resource);"] # [doc = ""] # [doc = " let expected = r#\"unnormalized-message ="] # [doc = "     This message has"] # [doc = "     abnormal spacing and indentation"] # [doc = " \"#;"] # [doc = ""] # [doc = " assert_eq!(expected, serialized);"] # [doc = " ```"] pub fn serialize < 's , S : Slice < 's > > (resource : & Resource < S >) -> String { serialize_with_options (resource , Options :: default ()) }
    };
}

serialize!()