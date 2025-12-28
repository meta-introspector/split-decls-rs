macro_rules! deps {
    () => {
        ArgScopeStack!();
        Demangle!();
        CloneTypeIdentifier!();
        DemangleContext!();
        Result!();
        DemangleWrite!();
    };
}

macro_rules! impl_133 {
    () => {
        deps!();
        impl < 'subs , W > Demangle < 'subs , W > for CloneTypeIdentifier where W : 'subs + DemangleWrite , { # [inline] fn demangle < 'prev , 'ctx > (& 'subs self , ctx : & 'ctx mut DemangleContext < 'subs , W > , scope : Option < ArgScopeStack < 'prev , 'subs > > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , scope) ; let ident = & ctx . input [self . start .. self . end] ; let source_name = String :: from_utf8_lossy (ident) ; ctx . set_source_name (self . start , self . end) ; write ! (ctx , " .{}" , source_name) ? ; Ok (()) } }
    };
}

impl_133!()