macro_rules! deps {
    () => {
        FileProcessingStatus!();
    };
}

macro_rules! FileProcessingResult {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Debug)] pub struct FileProcessingResult { pub path : PathBuf , pub status : FileProcessingStatus , }
    };
}

FileProcessingResult!()