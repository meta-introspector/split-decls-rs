macro_rules! deps {
    () => {
        VersionFileId!();
        VersionFiles!();
        ByteString!();
    };
}

macro_rules! VersionFile {
    () => {
        deps!();
        # [doc = " A filename used for GNU versioning."] # [doc = ""] # [doc = " Stored in [`VersionFiles`]."] # [derive (Debug)] pub struct VersionFile < 'data > { id : VersionFileId , # [doc = " Ignore this file when writing the ELF file."] pub delete : bool , # [doc = " The filename."] pub name : ByteString < 'data > , }
    };
}

VersionFile!();