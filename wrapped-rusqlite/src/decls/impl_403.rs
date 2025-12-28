macro_rules! deps {
    () => {
        ToSqlOutput!();
        ValueRef!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < 'a , T : ? Sized > From < & 'a T > for ToSqlOutput < 'a > where & 'a T : Into < ValueRef < 'a > > , { # [inline] fn from (t : & 'a T) -> Self { ToSqlOutput :: Borrowed (t . into ()) } }
    };
}

impl_403!();