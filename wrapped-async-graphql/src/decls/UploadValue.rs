macro_rules! UploadValue {
    () => {
        # [doc = " A file upload value."] pub struct UploadValue { # [doc = " The name of the file."] pub filename : String , # [doc = " The content type of the file."] pub content_type : Option < String > , # [doc = " The file data."] # [cfg (feature = "tempfile")] pub content : std :: fs :: File , # [doc = " The file data."] # [cfg (not (feature = "tempfile"))] pub content : bytes :: Bytes , }
    };
}

UploadValue!();