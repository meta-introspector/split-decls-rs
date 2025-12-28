macro_rules! deps {
    () => {
        BuiltinAttribute!();
    };
}

macro_rules! BUILTIN_ATTRIBUTE_MAP {
    () => {
        deps!();
        pub static BUILTIN_ATTRIBUTE_MAP : LazyLock < FxHashMap < Symbol , & BuiltinAttribute > > = LazyLock :: new (| | { let mut map = FxHashMap :: default () ; for attr in BUILTIN_ATTRIBUTES . iter () { if map . insert (attr . name , attr) . is_some () { panic ! ("duplicate builtin attribute `{}`" , attr . name) ; } } map }) ;
    };
}

BUILTIN_ATTRIBUTE_MAP!();