macro_rules! LiveFile {
    () => {
        # [doc = " The metadata that describes a SST file"] # [derive (Debug , Clone)] pub struct LiveFile { # [doc = " Name of the column family the file belongs to"] pub column_family_name : String , # [doc = " Name of the file"] pub name : String , # [doc = " Size of the file"] pub size : usize , # [doc = " Level at which this file resides"] pub level : i32 , # [doc = " Smallest user defined key in the file"] pub start_key : Option < Vec < u8 > > , # [doc = " Largest user defined key in the file"] pub end_key : Option < Vec < u8 > > , # [doc = " Number of entries/alive keys in the file"] pub num_entries : u64 , # [doc = " Number of deletions/tomb key(s) in the file"] pub num_deletions : u64 , }
    };
}

LiveFile!();