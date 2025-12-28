macro_rules! deps {
    () => {
        IndexConstraintUsage!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl IndexConstraintUsage < '_ > { # [doc = " if `argv_index` > 0, constraint is part of argv to"] # [doc = " [`VTabCursor::filter`]"] # [inline] pub fn set_argv_index (& mut self , argv_index : c_int) { self . 0 . argvIndex = argv_index ; } # [doc = " if `omit`, do not code a test for this constraint"] # [inline] pub fn set_omit (& mut self , omit : bool) { self . 0 . omit = omit as std :: ffi :: c_uchar ; } }
    };
}

impl_557!();