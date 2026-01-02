mkuse!{use backtrace :: Backtrace ;}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { println ! ("{:?}" , Backtrace :: new ()) ; }
}