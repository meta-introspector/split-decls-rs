macro_rules! deps {
    () => {
        InputValueDefinition!();
        Positioned!();
        DirectiveLocation!();
    };
}

macro_rules! DirectiveDefinition {
    () => {
        deps!();
        # [doc = " The definition of a directive in a service."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#DirectiveDefinition)."] # [derive (Debug , Clone)] pub struct DirectiveDefinition { # [doc = " The description of the directive."] pub description : Option < Positioned < String > > , # [doc = " The name of the directive."] pub name : Positioned < Name > , # [doc = " The arguments of the directive."] pub arguments : Vec < Positioned < InputValueDefinition > > , # [doc = " Whether the directive can be repeated."] pub is_repeatable : bool , # [doc = " The locations the directive applies to."] pub locations : Vec < Positioned < DirectiveLocation > > , }
    };
}

DirectiveDefinition!();