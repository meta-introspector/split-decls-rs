macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! ImportTable {
    () => {
        deps!();
        # [doc = " Information for parsing a PE import table."] # [doc = ""] # [doc = " Returned by [`DataDirectories::import_table`](super::DataDirectories::import_table)."] # [derive (Debug , Clone)] pub struct ImportTable < 'data > { section_data : Bytes < 'data > , section_address : u32 , import_address : u32 , }
    };
}

ImportTable!()