macro_rules! deps {
    () => {
        ImportLibraryItem!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl ImportLibraryItem { fn into_coff_short_export (self , sess : & Session) -> COFFShortExport { let import_name = (sess . target . arch == "arm64ec") . then (| | self . name . clone ()) ; COFFShortExport { name : self . name , ext_name : None , symbol_name : self . symbol_name , import_name , export_as : None , ordinal : self . ordinal . unwrap_or (0) , noname : self . ordinal . is_some () , data : self . is_data , private : false , constant : false , } } }
    };
}

impl_22!();