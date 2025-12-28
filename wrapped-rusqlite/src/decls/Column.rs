macro_rules! Column {
    () => {
        # [doc = " Information about a column of a SQLite query."] # [cfg (feature = "column_decltype")] # [derive (Debug)] pub struct Column < 'stmt > { name : & 'stmt str , decl_type : Option < & 'stmt str > , }
    };
}

Column!()