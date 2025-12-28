macro_rules! deps {
    () => {
        IndexConstraintOp!();
    };
}

macro_rules! impl_545 {
    () => {
        deps!();
        impl From < u8 > for IndexConstraintOp { fn from (code : u8) -> Self { match code { 2 => Self :: SQLITE_INDEX_CONSTRAINT_EQ , 4 => Self :: SQLITE_INDEX_CONSTRAINT_GT , 8 => Self :: SQLITE_INDEX_CONSTRAINT_LE , 16 => Self :: SQLITE_INDEX_CONSTRAINT_LT , 32 => Self :: SQLITE_INDEX_CONSTRAINT_GE , 64 => Self :: SQLITE_INDEX_CONSTRAINT_MATCH , 65 => Self :: SQLITE_INDEX_CONSTRAINT_LIKE , 66 => Self :: SQLITE_INDEX_CONSTRAINT_GLOB , 67 => Self :: SQLITE_INDEX_CONSTRAINT_REGEXP , 68 => Self :: SQLITE_INDEX_CONSTRAINT_NE , 69 => Self :: SQLITE_INDEX_CONSTRAINT_ISNOT , 70 => Self :: SQLITE_INDEX_CONSTRAINT_ISNOTNULL , 71 => Self :: SQLITE_INDEX_CONSTRAINT_ISNULL , 72 => Self :: SQLITE_INDEX_CONSTRAINT_IS , 73 => Self :: SQLITE_INDEX_CONSTRAINT_LIMIT , 74 => Self :: SQLITE_INDEX_CONSTRAINT_OFFSET , v => Self :: SQLITE_INDEX_CONSTRAINT_FUNCTION (v) , } } }
    };
}

impl_545!()