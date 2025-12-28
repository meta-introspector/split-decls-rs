macro_rules! deps {
    () => {
        OnDiskFileState!();
    };
}

macro_rules! OnDiskFile {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct OnDiskFile < T : Clone > { # [doc = " The last known path of the file"] path : Arc < PathBuf > , # [doc = " the time the file was last modified"] mtime : SystemTime , state : OnDiskFileState < T > , }
    };
}

OnDiskFile!();