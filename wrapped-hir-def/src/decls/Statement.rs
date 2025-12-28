macro_rules! deps {
    () => {
        Expr!();
        ExprId!();
        Item!();
        PatId!();
    };
}

macro_rules! Statement {
    () => {
        deps!();
        # [derive (Debug , Clone , Eq , PartialEq)] pub enum Statement { Let { pat : PatId , type_ref : Option < TypeRefId > , initializer : Option < ExprId > , else_branch : Option < ExprId > , } , Expr { expr : ExprId , has_semi : bool , } , Item (Item) , }
    };
}

Statement!();