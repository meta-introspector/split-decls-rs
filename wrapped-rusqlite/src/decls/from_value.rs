macro_rules! deps {
    () => {
        ToSqlOutput!();
    };
}

macro_rules! from_value {
    () => {
        deps!();
        macro_rules ! from_value (($ t : ty) => (impl From <$ t > for ToSqlOutput <'_ > { # [inline] fn from (t : $ t) -> Self { ToSqlOutput :: Owned (t . into ()) } }) ; (non_zero $ t : ty) => (impl From <$ t > for ToSqlOutput <'_ > { # [inline] fn from (t : $ t) -> Self { ToSqlOutput :: Owned (t . get () . into ()) } })) ;
    };
}

from_value!()