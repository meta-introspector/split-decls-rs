macro_rules! deps {
    () => {
        RenameRule!();
        RenameTarget!();
        Argument!();
        EnumItem!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl RenameTarget { fn rule (& self) -> RenameRule { match self { RenameTarget :: Type => RenameRule :: Pascal , RenameTarget :: EnumItem => RenameRule :: ScreamingSnake , RenameTarget :: Field => RenameRule :: Camel , RenameTarget :: Argument => RenameRule :: Camel , } } pub fn rename (& self , name : impl AsRef < str >) -> String { self . rule () . rename (name) } }
    };
}

impl_46!();