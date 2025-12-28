macro_rules! deps {
    () => {
        TomlProfiles!();
        TomlProfile!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl TomlProfiles { pub fn get_all (& self) -> & BTreeMap < ProfileName , TomlProfile > { & self . 0 } pub fn get (& self , name : & str) -> Option < & TomlProfile > { self . 0 . get (name) } }
    };
}

impl_121!()