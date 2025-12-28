macro_rules! deps {
    () => {
        Action!();
        Operation!();
    };
}

macro_rules! impl_257 {
    () => {
        deps!();
        impl Operation < '_ > { # [doc = " Returns the table name."] # [inline] pub fn table_name (& self) -> & str { self . table_name } # [doc = " Returns the number of columns in table"] # [inline] pub fn number_of_columns (& self) -> i32 { self . number_of_columns } # [doc = " Returns the action code."] # [inline] pub fn code (& self) -> Action { self . code } # [doc = " Returns `true` for an 'indirect' change."] # [inline] pub fn indirect (& self) -> bool { self . indirect } }
    };
}

impl_257!();