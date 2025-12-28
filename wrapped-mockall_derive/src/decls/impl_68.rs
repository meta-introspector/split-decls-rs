macro_rules! deps {
    () => {
        MockItemStruct!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl MockItemStruct { fn debug_impl (& self) -> impl ToTokens { if self . auto_debug { let (ig , tg , wc) = self . generics . split_for_impl () ; let struct_name = & self . name ; let struct_name_str = format ! ("{}" , self . name) ; quote ! (impl # ig :: std :: fmt :: Debug for # struct_name # tg # wc { fn fmt (& self , f : & mut :: std :: fmt :: Formatter <'_ >) -> :: std :: result :: Result < () , std :: fmt :: Error > { f . debug_struct (# struct_name_str) . finish () } }) } else { quote ! () } } fn new_method (& self) -> impl ToTokens { if self . has_new { TokenStream :: new () } else { quote ! (# [doc = " Create a new mock object with no expectations."] # [doc = ""] # [doc = " This method will not be generated if the real struct"] # [doc = " already has a `new` method.  However, it *will* be"] # [doc = " generated if the struct implements a trait with a `new`"] # [doc = " method.  The trait's `new` method can still be called"] # [doc = " like `<MockX as TraitY>::new`"] pub fn new () -> Self { Self :: default () }) } } fn phantom_default_inits (& self) -> Vec < TokenStream > { phantom_default_inits (& self . generics) } fn phantom_fields (& self) -> Vec < TokenStream > { phantom_fields (& self . generics) } }
    };
}

impl_68!();