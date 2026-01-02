mkitem!{type Pos = (& 'static str , u32) ;}
mkitem!{macro_rules ! pos { () => { (file ! () , line ! ()) } ; }}

macro_rules! foo_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function foo in module {}", module_path!());
    };
}

mkfn!{
    foo_introspect!();
    # [unsafe (no_mangle)] pub extern "C" fn foo (outer : Pos , inner : fn (Pos , Pos)) { inner (outer , pos ! ()) ; }
}