
macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { foo () ; }
}

macro_rules! foo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function foo in module {}", module_path!());
    };
}

mkfn!{
    foo_introspect!();
    fn foo () { bar () }
}

macro_rules! bar_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function bar in module {}", module_path!());
    };
}

mkfn!{
    bar_introspect!();
    fn bar () { baz () }
}

macro_rules! baz_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function baz in module {}", module_path!());
    };
}

mkfn!{
    baz_introspect!();
    fn baz () { print () }
}
mkitem!{# [cfg (target_pointer_width = "32")] const HEX_WIDTH : usize = 10 ;}
mkitem!{# [cfg (target_pointer_width = "64")] const HEX_WIDTH : usize = 20 ;}

macro_rules! print_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function print in module {}", module_path!());
    };
}

mkfn!{
    print_introspect!();
    fn print () { let mut cnt = 0 ; backtrace :: trace (| frame | { let ip = frame . ip () ; print ! ("frame #{:<2} - {:#02$x}" , cnt , ip as usize , HEX_WIDTH) ; cnt += 1 ; let mut resolved = false ; backtrace :: resolve (frame . ip () , | symbol | { if ! resolved { resolved = true ; } else { print ! ("{}" , vec ! [" " ; 7 + 2 + 3 + HEX_WIDTH] . join ("")) ; } if let Some (name) = symbol . name () { print ! (" - {name}") ; } else { print ! (" - <unknown>") ; } if let Some (file) = symbol . filename () { if let Some (l) = symbol . lineno () { print ! ("\n{:13}{:4$}@ {}:{}" , "" , "" , file . display () , l , HEX_WIDTH) ; } } println ! ("") ; }) ; if ! resolved { println ! (" - <no info>") ; } true }) ; }
}