macro_rules! deps {
    () => {
        Fail!();
        Error!();
        Marker!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl Marker { # [doc = " Like [`acquire_to_update_resource()`](File::acquire_to_update_resource()) but _without_ the possibility to make changes"] # [doc = " and commit them."] # [doc = ""] # [doc = " If `boundary_directory` is given, non-existing directories will be created automatically and removed in the case of"] # [doc = " a rollback."] # [doc = ""] # [doc = " Note that permissions will be set to `0o666`, which usually results in `0o644` after passing a default umask, on Unix systems."] # [doc = ""] # [doc = " ### Warning of potential resource leak"] # [doc = ""] # [doc = " Please note that the underlying file will remain if destructors don't run, as is the case when interrupting the application."] # [doc = " This results in the resource being locked permanently unless the lock file is removed by other means."] # [doc = " See [the crate documentation](crate) for more information."] pub fn acquire_to_hold_resource (at_path : impl AsRef < Path > , mode : Fail , boundary_directory : Option < PathBuf > ,) -> Result < Marker , Error > { let (lock_path , handle) = lock_with_mode (at_path . as_ref () , mode , boundary_directory , & | p , d , c | { if let Some (permissions) = default_permissions () { gix_tempfile :: mark_at_with_permissions (p , d , c , permissions) } else { gix_tempfile :: mark_at (p , d , c) } }) ? ; Ok (Marker { created_from_file : false , inner : handle , lock_path , }) } # [doc = " Like [`acquire_to_hold_resource()`](Marker::acquire_to_hold_resource), but allows to set filesystem permissions using `make_permissions`."] pub fn acquire_to_hold_resource_with_permissions (at_path : impl AsRef < Path > , mode : Fail , boundary_directory : Option < PathBuf > , make_permissions : impl Fn () -> std :: fs :: Permissions ,) -> Result < Marker , Error > { let (lock_path , handle) = lock_with_mode (at_path . as_ref () , mode , boundary_directory , & | p , d , c | { gix_tempfile :: mark_at_with_permissions (p , d , c , make_permissions ()) }) ? ; Ok (Marker { created_from_file : false , inner : handle , lock_path , }) } }
    };
}

impl_6!()