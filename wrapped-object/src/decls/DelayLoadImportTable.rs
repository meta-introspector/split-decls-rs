macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! DelayLoadImportTable {
    () => {
        deps!();
        # [doc = " Information for parsing a PE delay-load import table."] # [doc = ""] # [doc = " Returned by"] # [doc = " [`DataDirectories::delay_load_import_table`](super::DataDirectories::delay_load_import_table)."] # [derive (Debug , Clone)] pub struct DelayLoadImportTable < 'data > { section_data : Bytes < 'data > , section_address : u32 , import_address : u32 , }
    };
}

DelayLoadImportTable!()