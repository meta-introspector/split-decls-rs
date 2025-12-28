macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! ConstDirective {
    () => {
        deps!();
        # [doc = " A const GraphQL directive, such as `@deprecated(reason: \"Use the other"] # [doc = " field)`. This differs from [`Directive`](struct.Directive.html) in that it"] # [doc = " uses [`ConstValue`](enum.ConstValue.html) instead of"] # [doc = " [`Value`](enum.Value.html)."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#Directive)."] # [derive (Debug , Clone)] pub struct ConstDirective { # [doc = " The name of the directive."] pub name : Positioned < Name > , # [doc = " The arguments to the directive."] pub arguments : Vec < (Positioned < Name > , Positioned < ConstValue >) > , }
    };
}

ConstDirective!()