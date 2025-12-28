macro_rules! deps {
    () => {
        GrammarSource!();
    };
}

macro_rules! get_attribute {
    () => {
        deps!();
        fn get_attribute (attr : & Attribute) -> GrammarSource { match & attr . meta { Meta :: NameValue (name_value) => match & name_value . value { Expr :: Lit (ExprLit { lit : Lit :: Str (string) , .. }) => { if name_value . path . is_ident ("grammar") { GrammarSource :: File (string . value ()) } else { GrammarSource :: Inline (string . value ()) } } _ => panic ! ("grammar attribute must be a string") , } , _ => panic ! ("grammar attribute must be of the form `grammar = \"...\"`") , } }
    };
}

get_attribute!();