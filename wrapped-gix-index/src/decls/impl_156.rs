macro_rules! deps {
    () => {
        Metadata!();
        Error!();
        File!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        # [doc = " Lifecycle"] impl Metadata { # [doc = " Obtain the metadata at `path` without following symlinks."] pub fn from_path_no_follow (path : & Path) -> Result < Self , std :: io :: Error > { # [cfg (not (windows))] { rustix :: fs :: lstat (path) . map (Metadata) . map_err (Into :: into) } # [cfg (windows)] path . symlink_metadata () . map (Metadata) } # [doc = " Obtain the metadata at `path` without following symlinks."] pub fn from_file (file : & std :: fs :: File) -> Result < Self , std :: io :: Error > { # [cfg (not (windows))] { rustix :: fs :: fstat (file) . map (Metadata) . map_err (Into :: into) } # [cfg (windows)] file . metadata () . map (Metadata) } }
    };
}

impl_156!();