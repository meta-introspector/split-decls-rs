macro_rules! deps {
    () => {
        TypeName!();
    };
}

macro_rules! TypeRef {
    () => {
        deps!();
        pub struct TypeRef { pub ResolutionScope : ResolutionScope , pub TypeName : id :: StringId , pub TypeNamespace : id :: StringId , }
    };
}

TypeRef!();