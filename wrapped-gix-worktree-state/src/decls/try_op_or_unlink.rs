macro_rules! try_op_or_unlink {
    () => {
        # [doc = " Note that this works only because we assume to not race ourselves when symlinks are involved, and we do this by"] # [doc = " delaying symlink creation to the end and will always do that sequentially."] # [doc = " It's still possible to fall for a race if other actors create symlinks in our path, but that's nothing to defend against."] fn try_op_or_unlink < T > (path : & Path , overwrite_existing : bool , op : impl Fn (& Path) -> std :: io :: Result < T > ,) -> std :: io :: Result < T > { if overwrite_existing { match op (path) { Ok (res) => Ok (res) , Err (err) if gix_fs :: symlink :: is_collision_error (& err) => { try_unlink_path_recursively (path , & std :: fs :: symlink_metadata (path) ?) ? ; op (path) } Err (err) => Err (err) , } } else { op (path) } }
    };
}

try_op_or_unlink!();