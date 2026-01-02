mkitem!{mktrait!{trait Service { type S ; }}}
mkitem!{mktrait!{trait Framing { type F ; }}}
mkitem!{mkimpl!{impl Framing for () { type F = () ; }}}
mkitem!{mktrait!{trait HttpService < F : Framing > : Service < S = F :: F > { }}}
mkitem!{type BoxService = Box < dyn HttpService < () , S = () > > ;}

macro_rules! build_server_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function build_server in module {}", module_path!());
    };
}

mkfn!{
    build_server_introspect!();
    fn build_server < F : FnOnce () -> BoxService > (_ : F) { }
}

macro_rules! make_server_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_server in module {}", module_path!());
    };
}

mkfn!{
    make_server_introspect!();
    fn make_server < F : Framing > () -> Box < dyn HttpService < F , S = F :: F > > { unimplemented ! () }
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { build_server (| | make_server ()) }
}