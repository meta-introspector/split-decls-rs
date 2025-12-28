macro_rules! deps {
    () => {
        FormatArgument!();
        Walkable!();
    };
}

macro_rules! FormatArguments {
    () => {
        deps!();
        # [doc = " The arguments to format_args!()."] # [doc = ""] # [doc = " E.g. `1, 2, name=\"ferris\", n=3`,"] # [doc = " but also implicit captured arguments like `x` in `format_args!(\"{x}\")`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub struct FormatArguments { arguments : Vec < FormatArgument > , num_unnamed_args : usize , num_explicit_args : usize , names : FxHashMap < Symbol , usize > , }
    };
}

FormatArguments!()