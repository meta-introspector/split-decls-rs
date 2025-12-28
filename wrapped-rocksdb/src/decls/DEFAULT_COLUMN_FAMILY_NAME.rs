macro_rules! DEFAULT_COLUMN_FAMILY_NAME {
    () => {
        # [doc = " The name of the default column family."] # [doc = ""] # [doc = " The column family with this name is created implicitly whenever column"] # [doc = " families are used."] pub const DEFAULT_COLUMN_FAMILY_NAME : & str = "default" ;
    };
}

DEFAULT_COLUMN_FAMILY_NAME!();