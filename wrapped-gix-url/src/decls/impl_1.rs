macro_rules! deps {
    () => {
        ForUser!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl From < ForUser > for Option < BString > { fn from (v : ForUser) -> Self { match v { ForUser :: Name (user) => Some (user) , ForUser :: Current => None , } } }
    };
}

impl_1!();