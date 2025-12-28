macro_rules! deps {
    () => {
        RuleType!();
        ParserNode!();
    };
}

macro_rules! ParserRule {
    () => {
        deps!();
        # [doc = " The pest grammar rule"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct ParserRule < 'i > { # [doc = " The rule's name"] pub name : String , # [doc = " The rule's span"] pub span : Span < 'i > , # [doc = " The rule's type"] pub ty : RuleType , # [doc = " The rule's parser node"] pub node : ParserNode < 'i > , }
    };
}

ParserRule!()