macro_rules! deps {
    () => {
        Demangle!();
        Result!();
        DemangleContext!();
        ArgScopeStack!();
        Identifier!();
        DemangleWrite!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for Identifier where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let ident = & ctx . input [self . start .. self . end] ; let anon_namespace_prefix = b"_GLOBAL_" ; if ident . starts_with (anon_namespace_prefix) && ident . len () >= anon_namespace_prefix . len () + 2 { let first = ident [anon_namespace_prefix . len ()] ; let second = ident [anon_namespace_prefix . len () + 1] ; match (first , second) { (b'.' , b'N') | (b'_' , b'N') | (b'$' , b'N') => { write ! (ctx , "(anonymous namespace)") ? ; return Ok (()) ; } _ => { } } } let source_name = String :: from_utf8_lossy (ident) ; ctx . set_source_name (self . start , self . end) ; write ! (ctx , "{}" , source_name) ? ; Ok (()) } }
    };
}

impl_130!();