macro_rules! ColumnMetadata {
    () => {
        # [doc = " Metadata about the origin of a column of a SQLite query"] # [cfg (feature = "column_metadata")] # [derive (Debug)] pub struct ColumnMetadata < 'stmt > { name : & 'stmt str , database_name : Option < & 'stmt str > , table_name : Option < & 'stmt str > , origin_name : Option < & 'stmt str > , }
    };
}

ColumnMetadata!()