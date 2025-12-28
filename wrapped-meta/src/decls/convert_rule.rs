macro_rules! deps {
    () => {
        ParserRule!();
    };
}

macro_rules! convert_rule {
    () => {
        deps!();
        fn convert_rule (rule : ParserRule < '_ >) -> AstRule { let ParserRule { name , ty , node , .. } = rule ; let expr = convert_node (node) ; AstRule { name , ty , expr } }
    };
}

convert_rule!()