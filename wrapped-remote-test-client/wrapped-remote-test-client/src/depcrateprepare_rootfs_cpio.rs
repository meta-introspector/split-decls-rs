// Generated macro for prepare_rootfs_cpio (function)
macro_rules! Depcrateprepare_rootfs_cpio {
() => {
// Module: crate
// Provides: {"prepare_rootfs_cpio"}
// Dependencies: {}
fn prepare_rootfs_cpio (rootfs : & Path , rootfs_img : & Path) { let mut cmd = Command :: new ("cpio") ; cmd . arg ("--null") . arg ("-o") . arg ("--format=newc") . stdin (Stdio :: piped ()) . stdout (Stdio :: piped ()) . current_dir (rootfs) ; let mut child = t ! (cmd . spawn ()) ; let mut stdin = child . stdin . take () . unwrap () ; let rootfs = rootfs . to_path_buf () ; thread :: spawn (move | | add_files (& mut stdin , & rootfs , & rootfs)) ; t ! (io :: copy (& mut child . stdout . take () . unwrap () , & mut t ! (File :: create (& rootfs_img)))) ; assert ! (t ! (child . wait ()) . success ()) ; fn add_files (w : & mut dyn Write , root : & Path , cur : & Path) { for entry in t ! (cur . read_dir ()) { let entry = t ! (entry) ; let path = entry . path () ; let to_print = path . strip_prefix (root) . unwrap () ; t ! (write ! (w , "{}\u{0}" , to_print . to_str () . unwrap ())) ; if t ! (entry . file_type ()) . is_dir () { add_files (w , root , & path) ; } } } }
};
}
