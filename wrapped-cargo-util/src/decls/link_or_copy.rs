macro_rules! link_or_copy {
    () => {
        # [doc = " Hardlink (file) or symlink (dir) src to dst if possible, otherwise copy it."] # [doc = ""] # [doc = " If the destination already exists, it is removed before linking."] pub fn link_or_copy (src : impl AsRef < Path > , dst : impl AsRef < Path >) -> Result < () > { let src = src . as_ref () ; let dst = dst . as_ref () ; _link_or_copy (src , dst) }
    };
}

link_or_copy!();