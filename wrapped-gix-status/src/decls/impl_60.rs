macro_rules! deps {
    () => {
        Delegate!();
        Error!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl gix_fs :: stack :: Delegate for Delegate { fn push_directory (& mut self , _stack : & Stack) -> std :: io :: Result < () > { Ok (()) } # [cfg_attr (windows , allow (unused_variables))] fn push (& mut self , is_last_component : bool , stack : & Stack) -> std :: io :: Result < () > { # [cfg (windows)] { Ok (()) } # [cfg (not (windows))] { if is_last_component { return Ok (()) ; } if stack . current () . symlink_metadata () ? . is_symlink () { return Err (std :: io :: Error :: other ("Cannot step through symlink to perform an lstat")) ; } Ok (()) } } fn pop_directory (& mut self) { } }
    };
}

impl_60!();