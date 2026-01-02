mkuse!{use std :: marker :: Unsize ;}
mkuse!{use std :: ops :: { CoerceUnsized , Deref , DispatchFromDyn } ;}
mkitem!{mkstruct!{struct Ptr < T : ? Sized > (Box < T >) ;}}
mkitem!{mkimpl!{impl < T : ? Sized > Deref for Ptr < T > { type Target = T ; fn deref (& self) -> & T { & * self . 0 } }}}
mkitem!{mkimpl!{impl < T : Unsize < U > + ? Sized , U : ? Sized > CoerceUnsized < Ptr < U > > for Ptr < T > { }}}
mkitem!{mkimpl!{impl < T : Unsize < U > + ? Sized , U : ? Sized > DispatchFromDyn < Ptr < U > > for Ptr < T > { }}}
mkitem!{mkstruct!{struct Wrapper < T : ? Sized > (T) ;}}
mkitem!{mkimpl!{impl < T : ? Sized > Deref for Wrapper < T > { type Target = T ; fn deref (& self) -> & T { & self . 0 } }}}
mkitem!{mkimpl!{impl < T : CoerceUnsized < U > , U > CoerceUnsized < Wrapper < U > > for Wrapper < T > { }}}
mkitem!{mkimpl!{impl < T : DispatchFromDyn < U > , U > DispatchFromDyn < Wrapper < U > > for Wrapper < T > { }}}
mkitem!{mktrait!{trait Trait { fn ptr_wrapper (self : Ptr < Wrapper < Self > >) -> i32 ; fn wrapper_ptr (self : Wrapper < Ptr < Self > >) -> i32 ; fn wrapper_ptr_wrapper (self : Wrapper < Ptr < Wrapper < Self > > >) -> i32 ; }}}
mkitem!{mkimpl!{impl Trait for i32 { fn ptr_wrapper (self : Ptr < Wrapper < Self > >) -> i32 { * * self } fn wrapper_ptr (self : Wrapper < Ptr < Self > >) -> i32 { * * self } fn wrapper_ptr_wrapper (self : Wrapper < Ptr < Wrapper < Self > > >) -> i32 { * * * self } }}}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { let pw = Ptr (Box :: new (Wrapper (5))) as Ptr < Wrapper < dyn Trait > > ; assert_eq ! (pw . ptr_wrapper () , 5) ; let wp = Wrapper (Ptr (Box :: new (6))) as Wrapper < Ptr < dyn Trait > > ; assert_eq ! (wp . wrapper_ptr () , 6) ; let wpw = Wrapper (Ptr (Box :: new (Wrapper (7)))) as Wrapper < Ptr < Wrapper < dyn Trait > > > ; assert_eq ! (wpw . wrapper_ptr_wrapper () , 7) ; }
}