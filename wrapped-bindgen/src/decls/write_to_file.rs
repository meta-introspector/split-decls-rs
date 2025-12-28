macro_rules! write_to_file {
    () => {
        # [track_caller] pub fn write_to_file < C : AsRef < [u8] > > (path : & str , contents : C) { if let Some (parent) = std :: path :: Path :: new (path) . parent () { if std :: fs :: create_dir_all (parent) . is_err () { panic ! ("failed to create directory `{path}`") ; } } if std :: fs :: write (path , contents) . is_err () { panic ! ("failed to write file `{path}`") ; } }
    };
}

write_to_file!();