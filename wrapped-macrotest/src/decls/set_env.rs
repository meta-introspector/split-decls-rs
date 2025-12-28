macro_rules! set_env {
    () => {
        pub fn set_env (cmd : & mut Command) { let (key , mut val , separator) = match env :: var_os (CARGO_ENCODED_RUSTFLAGS) { Some (val) => (CARGO_ENCODED_RUSTFLAGS , val , "\x1f") , None => match env :: var_os (RUSTFLAGS) { Some (val) => (RUSTFLAGS , val , " ") , None => return , } , } ; for flag in make_vec () { if ! val . is_empty () { val . push (separator) ; } val . push (flag) ; } cmd . env (key , val) ; }
    };
}

set_env!();