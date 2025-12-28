macro_rules! CrateInfo {
    () => {
        # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct CrateInfo { pub name : Option < String > , pub version : Option < String > , pub root_file_id : FileId , }
    };
}

CrateInfo!();