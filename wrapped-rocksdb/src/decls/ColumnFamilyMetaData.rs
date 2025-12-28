macro_rules! ColumnFamilyMetaData {
    () => {
        # [doc = " The metadata that describes a column family."] # [derive (Debug , Clone)] pub struct ColumnFamilyMetaData { pub size : u64 , pub name : String , pub file_count : usize , }
    };
}

ColumnFamilyMetaData!();