macro_rules! is_path_owned_by_current_user {
    () => {
        # [doc = " Returns true if the given `path` is owned by the user who is executing the current process."] # [doc = ""] # [doc = " Note that this method is very specific to avoid having to deal with any operating system types."] pub fn is_path_owned_by_current_user (path : & Path) -> std :: io :: Result < bool > { impl_ :: is_path_owned_by_current_user (path) }
    };
}

is_path_owned_by_current_user!()