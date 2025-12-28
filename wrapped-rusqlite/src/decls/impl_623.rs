macro_rules! deps {
    () => {
        IndexInfo!();
        VTab!();
        CsvTab!();
        CsvTabCursor!();
        VTabConnection!();
        Result!();
        Error!();
        VTabConfig!();
    };
}

macro_rules! impl_623 {
    () => {
        deps!();
        unsafe impl < 'vtab > VTab < 'vtab > for CsvTab { type Aux = () ; type Cursor = CsvTabCursor < 'vtab > ; fn connect (db : & mut VTabConnection , _aux : Option < & () > , args : & [& [u8]] ,) -> Result < (String , Self) > { if args . len () < 4 { return Err (Error :: ModuleError ("no CSV file specified" . to_owned ())) ; } let mut vtab = Self { base : ffi :: sqlite3_vtab :: default () , filename : String :: new () , has_headers : false , delimiter : b',' , quote : b'"' , offset_first_row : csv :: Position :: new () , } ; let mut schema = None ; let mut n_col = None ; let args = & args [3 ..] ; for c_slice in args { let (param , value) = super :: parameter (c_slice) ? ; match param { "filename" => { if ! Path :: new (value) . exists () { return Err (Error :: ModuleError (format ! ("file '{value}' does not exist"))) ; } value . clone_into (& mut vtab . filename) ; } "schema" => { schema = Some (value . to_owned ()) ; } "columns" => { if let Ok (n) = value . parse :: < u16 > () { if n_col . is_some () { return Err (Error :: ModuleError ("more than one 'columns' parameter" . to_owned () ,)) ; } else if n == 0 { return Err (Error :: ModuleError ("must have at least one column" . to_owned () ,)) ; } n_col = Some (n) ; } else { return Err (Error :: ModuleError (format ! ("unrecognized argument to 'columns': {value}"))) ; } } "header" => { if let Some (b) = parse_boolean (value) { vtab . has_headers = b ; } else { return Err (Error :: ModuleError (format ! ("unrecognized argument to 'header': {value}"))) ; } } "delimiter" => { if let Some (b) = Self :: parse_byte (value) { vtab . delimiter = b ; } else { return Err (Error :: ModuleError (format ! ("unrecognized argument to 'delimiter': {value}"))) ; } } "quote" => { if let Some (b) = Self :: parse_byte (value) { if b == b'0' { vtab . quote = 0 ; } else { vtab . quote = b ; } } else { return Err (Error :: ModuleError (format ! ("unrecognized argument to 'quote': {value}"))) ; } } _ => { return Err (Error :: ModuleError (format ! ("unrecognized parameter '{param}'"))) ; } } } if vtab . filename . is_empty () { return Err (Error :: ModuleError ("no CSV file specified" . to_owned ())) ; } let mut cols : Vec < String > = Vec :: new () ; if vtab . has_headers || (n_col . is_none () && schema . is_none ()) { let mut reader = vtab . reader () ? ; if vtab . has_headers { { let headers = reader . headers () ? ; if n_col . is_none () && schema . is_none () { cols = headers . into_iter () . map (| header | escape_double_quote (header) . into_owned ()) . collect () ; } } vtab . offset_first_row = reader . position () . clone () ; } else { let mut record = csv :: ByteRecord :: new () ; if reader . read_byte_record (& mut record) ? { for (i , _) in record . iter () . enumerate () { cols . push (format ! ("c{i}")) ; } } } } else if let Some (n_col) = n_col { for i in 0 .. n_col { cols . push (format ! ("c{i}")) ; } } if cols . is_empty () && schema . is_none () { return Err (Error :: ModuleError ("no column specified" . to_owned ())) ; } if schema . is_none () { let mut sql = String :: from ("CREATE TABLE x(") ; for (i , col) in cols . iter () . enumerate () { sql . push ('"') ; sql . push_str (col) ; sql . push_str ("\" TEXT") ; if i == cols . len () - 1 { sql . push_str (");") ; } else { sql . push_str (", ") ; } } schema = Some (sql) ; } db . config (VTabConfig :: DirectOnly) ? ; Ok ((schema . unwrap () , vtab)) } fn best_index (& self , info : & mut IndexInfo) -> Result < () > { info . set_estimated_cost (1_000_000.) ; Ok (()) } fn open (& mut self) -> Result < CsvTabCursor < '_ > > { Ok (CsvTabCursor :: new (self . reader () ?)) } }
    };
}

impl_623!()