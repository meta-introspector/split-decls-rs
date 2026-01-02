mkuse!{use std :: fmt ;}
mkuse!{use std :: io :: { self , Write as _ } ;}
mkitem!{macro_rules ! safe_print { ($ ($ arg : tt) *) => { { $ crate :: print :: print (std :: format_args ! ($ ($ arg) *)) ; } } ; }}
mkitem!{macro_rules ! safe_println { ($ ($ arg : tt) *) => { safe_print ! ("{}\n" , std :: format_args ! ($ ($ arg) *)) } ; }}

macro_rules! print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print in module {}", module_path!());
    };
}

mkfn!{
    print_introspect!();
    pub (crate) fn print (args : fmt :: Arguments < '_ >) { if let Err (_) = io :: stdout () . write_fmt (args) { rustc_errors :: FatalError . raise () ; } }
}