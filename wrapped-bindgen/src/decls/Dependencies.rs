macro_rules! deps {
    () => {
        TypeMap!();
    };
}

macro_rules! Dependencies {
    () => {
        deps!();
        pub trait Dependencies { fn combine (& self , dependencies : & mut TypeMap) ; fn dependencies (& self) -> TypeMap { let mut dependencies = TypeMap :: new () ; self . combine (& mut dependencies) ; dependencies } }
    };
}

Dependencies!();