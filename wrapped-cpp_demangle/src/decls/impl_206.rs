macro_rules! deps {
    () => {
        Result!();
        DemangleContext!();
        UnnamedTypeName!();
        DemangleAsLeaf!();
        DemangleWrite!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl < 'subs , W > DemangleAsLeaf < 'subs , W > for UnnamedTypeName where W : 'subs + DemangleWrite , { fn demangle_as_leaf < 'me , 'ctx > (& 'me self , ctx : & 'ctx mut DemangleContext < 'subs , W > ,) -> fmt :: Result { let ctx = try_begin_demangle ! (self , ctx , None) ; if let Some (source_name) = ctx . source_name { write ! (ctx , "{}" , source_name) ? ; } else { write ! (ctx , "{{unnamed type#{}}}" , self . 0 . map_or (1 , | n | n + 1)) ? ; } Ok (()) } }
    };
}

impl_206!();