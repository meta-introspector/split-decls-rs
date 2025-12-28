macro_rules! deps {
    () => {
        Directive!();
        Positioned!();
        Type!();
    };
}

macro_rules! VariableDefinition {
    () => {
        deps!();
        # [doc = " A variable definition inside a list of variable definitions, for example"] # [doc = " `$name:String!`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#VariableDefinition)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct VariableDefinition { # [doc = " The name of the variable, without the preceding `$`."] pub name : Positioned < Name > , # [doc = " The type of the variable."] pub var_type : Positioned < Type > , # [doc = " The variable's directives."] pub directives : Vec < Positioned < Directive > > , # [doc = " The optional default value of the variable."] pub default_value : Option < Positioned < ConstValue > > , }
    };
}

VariableDefinition!();