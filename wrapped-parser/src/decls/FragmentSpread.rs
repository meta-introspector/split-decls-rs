macro_rules! deps {
    () => {
        Positioned!();
        Directive!();
    };
}

macro_rules! FragmentSpread {
    () => {
        deps!();
        # [doc = " A fragment selector, such as `... userFields`."] # [doc = ""] # [doc = " [Reference](https://spec.graphql.org/October2021/#FragmentSpread)."] # [derive (Debug , Clone , Serialize , Deserialize)] pub struct FragmentSpread { # [doc = " The name of the fragment being selected."] pub fragment_name : Positioned < Name > , # [doc = " The directives in the fragment selector."] pub directives : Vec < Positioned < Directive > > , }
    };
}

FragmentSpread!();