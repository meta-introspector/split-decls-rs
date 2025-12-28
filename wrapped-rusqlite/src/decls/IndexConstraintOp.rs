macro_rules! IndexConstraintOp {
    () => {
        # [doc = " Index constraint operator."] # [doc = " See [Virtual Table Constraint Operator Codes](https://sqlite.org/c3ref/c_index_constraint_eq.html) for details."] # [derive (Debug , Eq , PartialEq)] # [allow (missing_docs)] # [expect (non_camel_case_types)] pub enum IndexConstraintOp { SQLITE_INDEX_CONSTRAINT_EQ , SQLITE_INDEX_CONSTRAINT_GT , SQLITE_INDEX_CONSTRAINT_LE , SQLITE_INDEX_CONSTRAINT_LT , SQLITE_INDEX_CONSTRAINT_GE , SQLITE_INDEX_CONSTRAINT_MATCH , SQLITE_INDEX_CONSTRAINT_LIKE , SQLITE_INDEX_CONSTRAINT_GLOB , SQLITE_INDEX_CONSTRAINT_REGEXP , SQLITE_INDEX_CONSTRAINT_NE , SQLITE_INDEX_CONSTRAINT_ISNOT , SQLITE_INDEX_CONSTRAINT_ISNOTNULL , SQLITE_INDEX_CONSTRAINT_ISNULL , SQLITE_INDEX_CONSTRAINT_IS , SQLITE_INDEX_CONSTRAINT_LIMIT , SQLITE_INDEX_CONSTRAINT_OFFSET , SQLITE_INDEX_CONSTRAINT_FUNCTION (u8) , }
    };
}

IndexConstraintOp!();