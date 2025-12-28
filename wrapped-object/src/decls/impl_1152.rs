macro_rules! deps {
    () => {
        VersionFileId!();
        VersionFiles!();
        VersionFile!();
        ByteString!();
    };
}

macro_rules! impl_1152 {
    () => {
        deps!();
        impl < 'data > VersionFiles < 'data > { # [doc = " Add a new filename to the table."] pub fn add (& mut self , name : ByteString < 'data >) -> VersionFileId { let id = self . next_id () ; self . push (VersionFile { id , name , delete : false , }) ; id } }
    };
}

impl_1152!();