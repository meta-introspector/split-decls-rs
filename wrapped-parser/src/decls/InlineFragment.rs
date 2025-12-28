macro_rules! deps {
    () => {
        Directive!();
        Positioned!();
        SelectionSet!();
        TypeCondition!();
    };
}

macro_rules! InlineFragment {
    () => {
        deps!();
        # [doc = " An inline fragment selector, such as `... on User { name }`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#InlineFragment)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct InlineFragment { # [doc = " The type condition."] pub type_condition : Option < Positioned < TypeCondition > > , # [doc = " The directives in the inline fragment."] pub directives : Vec < Positioned < Directive > > , # [doc = " The selected fields of the fragment."] pub selection_set : Positioned < SelectionSet > , }
    };
}

InlineFragment!();