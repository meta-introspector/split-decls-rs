macro_rules! deps {
    () => {
        U32!();
    };
}

macro_rules! ImageResourceDirectoryEntry {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageResourceDirectoryEntry { pub name_or_id : U32 < LE > , pub offset_to_data_or_directory : U32 < LE > , }
    };
}

ImageResourceDirectoryEntry!()