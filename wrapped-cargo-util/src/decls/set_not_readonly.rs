macro_rules! set_not_readonly {
    () => {
        fn set_not_readonly (p : & Path) -> io :: Result < bool > { let mut perms = p . metadata () ? . permissions () ; if ! perms . readonly () { return Ok (false) ; } perms . set_readonly (false) ; fs :: set_permissions (p , perms) ? ; Ok (true) }
    };
}

set_not_readonly!();