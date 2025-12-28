macro_rules! FileProcessingStatus {
    () => {
        # [derive (Serialize , Deserialize , Debug)] pub enum FileProcessingStatus { Success , Skipped { reason : String } , Failed { error : String } , }
    };
}

FileProcessingStatus!()