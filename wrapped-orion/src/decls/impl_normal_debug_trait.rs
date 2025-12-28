macro_rules! impl_normal_debug_trait {
    () => {
        # [doc = " Macro that implements the `Debug` trait on a object called `$name`."] macro_rules ! impl_normal_debug_trait (($ name : ident) => (impl core :: fmt :: Debug for $ name { fn fmt (& self , f : & mut core :: fmt :: Formatter <'_ >) -> core :: fmt :: Result { write ! (f , "{} {:?}" , stringify ! ($ name) , & self . value [..]) } })) ;
    };
}

impl_normal_debug_trait!();