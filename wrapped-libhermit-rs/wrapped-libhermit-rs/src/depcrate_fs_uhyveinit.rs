// Generated macro for init (function)
macro_rules! Depcrate_fs_uhyveinit {
() => {
// Module: crate::fs::uhyve
// Provides: {"init"}
// Dependencies: {}
pub (crate) fn init () { info ! ("Try to initialize uhyve filesystem") ; let mount_str = fdt () . and_then (| fdt | { fdt . find_node ("/uhyve,mounts") . and_then (| node | node . property ("mounts")) . and_then (| property | property . as_str ()) }) ; if let Some (mount_str) = mount_str { assert_ne ! (mount_str . len () , 0 , "Invalid /uhyve,mounts node in FDT") ; for mount_point in mount_str . split ('\0') { info ! ("Mounting uhyve filesystem at {mount_point}") ; if let Err (errno) = fs :: FILESYSTEM . get () . unwrap () . mount (mount_point , Box :: new (UhyveDirectory :: new (Some (mount_point . to_owned ()))) ,) { assert_eq ! (errno , Errno :: Badf) ; debug ! ("Mounting of {mount_point} failed with {errno:?}. Creating missing parent folders") ; let (parent_path , _file_name) = mount_point . rsplit_once ('/') . unwrap () ; create_dir_recursive (parent_path , AccessPermission :: S_IRWXU) . unwrap () ; fs :: FILESYSTEM . get () . unwrap () . mount (mount_point , Box :: new (UhyveDirectory :: new (Some (mount_point . to_owned ()))) ,) . unwrap () ; } } } else { let mount_point = hermit_var_or ! ("UHYVE_MOUNT" , "/root") . to_string () ; info ! ("Mounting uhyve filesystem at {mount_point}") ; fs :: FILESYSTEM . get () . unwrap () . mount (& mount_point , Box :: new (UhyveDirectory :: new (Some (mount_point . clone ()))) ,) . expect ("Mount failed. Duplicate mount_point?") ; } }
};
}
