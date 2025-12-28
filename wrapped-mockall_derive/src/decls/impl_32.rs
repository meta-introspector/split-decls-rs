macro_rules! deps {
    () => {
        RefRfunc!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl ToTokens for RefRfunc < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let fn_params = & self . f . fn_params ; let (ig , tg , wc) = self . f . egenerics . split_for_impl () ; let lg = lifetimes_to_generics (& self . f . alifetimes) ; let owned_output = & self . f . owned_output ; # [cfg (not (feature = "nightly_derive"))] let default_err_msg = "Returning default values requires the \"nightly\" feature" ; # [cfg (feature = "nightly_derive")] let default_err_msg = "Can only return default values for types that impl std::Default" ; quote ! (enum Rfunc # ig # wc { Default (Option <# owned_output >) , Const (# owned_output) , _Phantom (Mutex < Box < dyn Fn (# (# fn_params ,) *) + :: std :: marker :: Send >>) } impl # ig Rfunc # tg # wc { fn call # lg (& self) -> std :: result :: Result <&# owned_output , &'static str > { match self { Rfunc :: Default (Some (ref __mockall_o)) => { :: std :: result :: Result :: Ok (__mockall_o) } , Rfunc :: Default (None) => { Err (# default_err_msg) } , Rfunc :: Const (ref __mockall_o) => { :: std :: result :: Result :: Ok (__mockall_o) } , Rfunc :: _Phantom (_) => unreachable ! () } } } # [allow (single_use_lifetimes)] impl # ig std :: default :: Default for Rfunc # tg # wc { fn default () -> Self { use :: mockall :: ReturnDefault ; Rfunc :: Default (:: mockall :: DefaultReturner ::<# owned_output > :: maybe_return_default ()) } }) . to_tokens (tokens) ; } }
    };
}

impl_32!()