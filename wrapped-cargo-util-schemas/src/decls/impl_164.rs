macro_rules! deps {
    () => {
        InheritableDependency!();
        TomlPlatform!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        impl TomlPlatform { pub fn dev_dependencies (& self) -> Option < & BTreeMap < PackageName , InheritableDependency > > { self . dev_dependencies . as_ref () . or (self . dev_dependencies2 . as_ref ()) } pub fn build_dependencies (& self) -> Option < & BTreeMap < PackageName , InheritableDependency > > { self . build_dependencies . as_ref () . or (self . build_dependencies2 . as_ref ()) } }
    };
}

impl_164!();