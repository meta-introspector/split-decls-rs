macro_rules! deps {
    () => {
        Expr!();
        Walkable!();
    };
}

macro_rules! StructRest {
    () => {
        deps!();
        # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum StructRest { # [doc = " `..x`."] Base (Box < Expr >) , # [doc = " `..`."] Rest (Span) , # [doc = " No trailing `..` or expression."] None , }
    };
}

StructRest!()