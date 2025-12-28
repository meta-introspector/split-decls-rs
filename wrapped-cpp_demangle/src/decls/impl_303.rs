macro_rules! deps {
    () => {
        DemangleWrite!();
        Demangle!();
        DemangleContext!();
        Result!();
        ResourceName!();
        ArgScopeStack!();
    };
}

macro_rules! impl_303 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for ResourceName where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let mut i = self . start ; while i < self . end { let ch = ctx . input [i] ; if ch == b'$' { i += 1 ; match ctx . input [i] { b'S' => write ! (ctx , "{}" , '/') ? , b'_' => write ! (ctx , "{}" , '.') ? , b'$' => write ! (ctx , "{}" , '$') ? , _ => { } } } else { write ! (ctx , "{}" , ch as char) ? ; } i += 1 ; } Ok (()) } }
    };
}

impl_303!();