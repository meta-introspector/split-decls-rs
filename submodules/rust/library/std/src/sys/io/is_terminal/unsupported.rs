
macro_rules! is_terminal_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function is_terminal in module {}", module_path!());
    };
}

mkfn!{
    is_terminal_introspect!();
    pub fn is_terminal < T > (_ : & T) -> bool { false }
}