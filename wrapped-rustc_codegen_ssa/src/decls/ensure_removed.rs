macro_rules! ensure_removed {
    () => {
        pub fn ensure_removed (dcx : DiagCtxtHandle < '_ > , path : & Path) { if let Err (e) = fs :: remove_file (path) { if e . kind () != io :: ErrorKind :: NotFound { dcx . err (format ! ("failed to remove {}: {}" , path . display () , e)) ; } } }
    };
}

ensure_removed!();