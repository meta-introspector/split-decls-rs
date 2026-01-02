mkuse!{use crate :: os :: fd :: { AsFd , AsRawFd } ;}

macro_rules! is_terminal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_terminal in module {}", module_path!());
    };
}

mkfn!{
    is_terminal_introspect!();
    pub fn is_terminal (fd : & impl AsFd) -> bool { let fd = fd . as_fd () ; hermit_abi :: isatty (fd . as_raw_fd ()) }
}