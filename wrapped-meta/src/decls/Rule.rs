macro_rules! deps {
    () => {
        RuleType!();
        Expr!();
    };
}

macro_rules! Rule {
    () => {
        deps!();
        # [doc = " A grammar rule"] # [derive (Clone , Debug , Eq , PartialEq)] pub struct Rule { # [doc = " The name of the rule"] pub name : String , # [doc = " The rule's type (silent, atomic, ...)"] pub ty : RuleType , # [doc = " The rule's expression"] pub expr : Expr , }
    };
}

Rule!()