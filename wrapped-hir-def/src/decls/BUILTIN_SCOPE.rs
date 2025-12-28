macro_rules! deps {
    () => {
        Visibility!();
        FxIndexMap!();
        BuiltinType!();
        PerNs!();
    };
}

macro_rules! BUILTIN_SCOPE {
    () => {
        deps!();
        pub (crate) static BUILTIN_SCOPE : LazyLock < FxIndexMap < Name , PerNs > > = LazyLock :: new (| | { BuiltinType :: all_builtin_types () . iter () . map (| (name , ty) | (name . clone () , PerNs :: types ((* ty) . into () , Visibility :: Public , None))) . collect () }) ;
    };
}

BUILTIN_SCOPE!();