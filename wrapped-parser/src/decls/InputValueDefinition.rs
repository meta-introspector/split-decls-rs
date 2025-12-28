macro_rules! deps {
    () => {
        Type!();
        Positioned!();
        ConstDirective!();
    };
}

macro_rules! InputValueDefinition {
    () => {
        deps!();
        # [doc = " The definition of an input value inside the arguments of a field."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#InputValueDefinition)."] # [derive (Debug , Clone)] pub struct InputValueDefinition { # [doc = " The description of the argument."] pub description : Option < Positioned < String > > , # [doc = " The name of the argument."] pub name : Positioned < Name > , # [doc = " The type of the argument."] pub ty : Positioned < Type > , # [doc = " The default value of the argument, if there is one."] pub default_value : Option < Positioned < ConstValue > > , # [doc = " The directives of the input value."] pub directives : Vec < Positioned < ConstDirective > > , }
    };
}

InputValueDefinition!();