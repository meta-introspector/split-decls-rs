// Generated macro for objects_from_files (function)
macro_rules! Depcrate_command_helpersobjects_from_files {
() => {
// Module: crate::command_helpers
// Provides: {"objects_from_files"}
// Dependencies: {}
# [doc = " Find the destination object path for each file in the input source files,"] # [doc = " and store them in the output Object."] pub (crate) fn objects_from_files (files : & [Arc < Path >] , dst : & Path) -> Result < Vec < Object > , Error > { let mut objects = Vec :: with_capacity (files . len ()) ; for file in files { let basename = file . file_name () . ok_or_else (| | { Error :: new (ErrorKind :: InvalidArgument , "No file_name for object file path!" ,) }) ? . to_string_lossy () ; let dirname = file . parent () . ok_or_else (| | { Error :: new (ErrorKind :: InvalidArgument , "No parent for object file path!" ,) }) ? . to_string_lossy () ; let mut hasher = hash_map :: DefaultHasher :: new () ; # [allow (clippy :: disallowed_methods)] let dirname = if let Some (root) = std :: env :: var_os ("CARGO_MANIFEST_DIR") { let root = root . to_string_lossy () ; Cow :: Borrowed (dirname . strip_prefix (& * root) . unwrap_or (& dirname)) } else { dirname } ; hasher . write (dirname . as_bytes ()) ; if let Some (extension) = file . extension () { hasher . write (extension . to_string_lossy () . as_bytes ()) ; } let obj = dst . join (format ! ("{:016x}-{}" , hasher . finish () , basename)) . with_extension ("o") ; match obj . parent () { Some (s) => fs :: create_dir_all (s) ? , None => { return Err (Error :: new (ErrorKind :: InvalidArgument , "dst is an invalid path with no parent" ,)) ; } } ; objects . push (Object :: new (file . to_path_buf () , obj)) ; } Ok (objects) }
};
}
