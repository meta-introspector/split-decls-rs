macro_rules! deps {
    () => {
        Expression!();
    };
}

macro_rules! Initializer {
    () => {
        deps!();
        # [doc = " The `<initializer>` production."] # [doc = ""] # [doc = " ```text"] # [doc = " <initializer> ::= pi <expression>* E # parenthesized initialization"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct Initializer (Vec < Expression >) ;
    };
}

Initializer!()