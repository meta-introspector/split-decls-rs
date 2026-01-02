mkitem!{mktrait!{pub trait T { type Item ; }}}
mkitem!{pub type Alias < 'a > = impl T < Item = & 'a () > ;}
mkitem!{mkstruct!{struct S ;}}
mkitem!{mkimpl!{impl < 'a > T for & 'a S { type Item = & 'a () ; }}}

macro_rules! filter_positive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function filter_positive in module {}", module_path!());
    };
}

mkfn!{
    filter_positive_introspect!();
    # [define_opaque (Alias)] pub fn filter_positive < 'a > () -> Alias < 'a > { & S }
}

macro_rules! with_positive_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function with_positive in module {}", module_path!());
    };
}

mkfn!{
    with_positive_introspect!();
    fn with_positive (fun : impl Fn (Alias < '_ >)) { fun (filter_positive ()) ; }
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { with_positive (| _ | ()) ; }
}