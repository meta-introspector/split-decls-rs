macro_rules! deps {
    () => {
        PathKind!();
        Ty!();
    };
}

macro_rules! Path {
    () => {
        deps!();
        # [doc = " A path, e.g., `::std::option::Option::<i32>` (global). Has support"] # [doc = " for type parameters."] # [derive (Clone)] pub (crate) struct Path { path : Vec < Symbol > , params : Vec < Box < Ty > > , kind : PathKind , }
    };
}

Path!()