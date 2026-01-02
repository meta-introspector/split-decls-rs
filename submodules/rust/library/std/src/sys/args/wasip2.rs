mkuse!{pub use super :: common :: Args ;}

macro_rules! args_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function args in module {}", module_path!());
    };
}

mkfn!{
    args_introspect!();
    # [doc = " Returns the command line arguments"] pub fn args () -> Args { Args :: new (wasip2 :: cli :: environment :: get_arguments () . into_iter () . map (| arg | arg . into ()) . collect ()) }
}