macro_rules! deps {
    () => {
        RenameRule!();
    };
}

macro_rules! impl_44 {
    () => {
        deps!();
        impl RenameRule { fn rename (& self , name : impl AsRef < str >) -> String { match self { Self :: Lower => name . as_ref () . to_lowercase () , Self :: Upper => name . as_ref () . to_uppercase () , Self :: Pascal => name . as_ref () . to_pascal_case () , Self :: Camel => name . as_ref () . to_camel_case () , Self :: Snake => name . as_ref () . to_snake_case () , Self :: ScreamingSnake => name . as_ref () . to_screaming_snake_case () , } } }
    };
}

impl_44!();