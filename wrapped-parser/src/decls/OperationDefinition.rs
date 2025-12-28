macro_rules! deps {
    () => {
        Directive!();
        VariableDefinition!();
        SelectionSet!();
        OperationType!();
        Positioned!();
    };
}

macro_rules! OperationDefinition {
    () => {
        deps!();
        # [doc = " A GraphQL operation, such as `mutation($content:String!) { makePost(content:"] # [doc = " $content) { id } }`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#OperationDefinition)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct OperationDefinition { # [doc = " The type of operation."] pub ty : OperationType , # [doc = " The variable definitions."] pub variable_definitions : Vec < Positioned < VariableDefinition > > , # [doc = " The operation's directives."] pub directives : Vec < Positioned < Directive > > , # [doc = " The operation's selection set."] pub selection_set : Positioned < SelectionSet > , }
    };
}

OperationDefinition!();