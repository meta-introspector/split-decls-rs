macro_rules! deps {
    () => {
        Expression!();
        ExprPrimary!();
        Type!();
    };
}

macro_rules! TemplateArg {
    () => {
        deps!();
        # [doc = " A <template-arg> production."] # [doc = ""] # [doc = " ```text"] # [doc = " <template-arg> ::= <type>                # type or template"] # [doc = "                ::= X <expression> E      # expression"] # [doc = "                ::= <expr-primary>        # simple expressions"] # [doc = "                ::= J <template-arg>* E   # argument pack"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum TemplateArg { # [doc = " A type or template."] Type (TypeHandle) , # [doc = " An expression."] Expression (Expression) , # [doc = " A simple expression."] SimpleExpression (ExprPrimary) , # [doc = " An argument pack."] ArgPack (Vec < TemplateArg >) , }
    };
}

TemplateArg!()