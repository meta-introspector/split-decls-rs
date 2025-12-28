macro_rules! remove_symlink_dir_with_permission_check {
    () => {
        # [cfg (target_os = "windows")] fn remove_symlink_dir_with_permission_check (p : & Path) -> Result < () > { remove_with_permission_check (fs :: remove_dir , p) . with_context (| | format ! ("failed to remove symlink dir `{}`" , p . display ())) }
    };
}

remove_symlink_dir_with_permission_check!();