macro_rules! is_dot_git_ntfs {
    () => {
        fn is_dot_git_ntfs (input : & BStr) -> bool { if input . get (.. 4) . is_some_and (| input | input . eq_ignore_ascii_case (b".git")) { return is_done_ntfs (input . get (4 ..)) ; } if input . get (.. 5) . is_some_and (| input | input . eq_ignore_ascii_case (b"git~1")) { return is_done_ntfs (input . get (5 ..)) ; } false }
    };
}

is_dot_git_ntfs!();