macro_rules! FilePositionWrapper {
    () => {
        # [derive (Debug , PartialEq , Eq , Clone , Copy , Hash)] pub struct FilePositionWrapper < FileKind > { pub file_id : FileKind , pub offset : TextSize , }
    };
}

FilePositionWrapper!();