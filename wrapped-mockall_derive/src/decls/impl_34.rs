macro_rules! deps {
    () => {
        RefMutRfunc!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl ToTokens for RefMutRfunc < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let argnames = & self . f . argnames ; let argty = & self . f . argty ; let fn_params = & self . f . fn_params ; let (ig , tg , wc) = self . f . egenerics . split_for_impl () ; let lg = lifetimes_to_generics (& self . f . alifetimes) ; let owned_output = & self . f . owned_output ; let output = & self . f . output ; # [cfg (not (feature = "nightly_derive"))] let default_err_msg = "Returning default values requires the \"nightly\" feature" ; # [cfg (feature = "nightly_derive")] let default_err_msg = "Can only return default values for types that impl std::Default" ; quote ! (# [allow (clippy :: unused_unit)] enum Rfunc # ig # wc { Default (Option <# owned_output >) , Mut ((Box < dyn FnMut (# (# argty ,) *) -> # owned_output + :: std :: marker :: Send + :: std :: marker :: Sync >) , Option <# owned_output >) , MutSt ((:: mockall :: Fragile < Box < dyn FnMut (# (# argty ,) *) -> # owned_output >>) , Option <# owned_output >) , Var (# owned_output) , _Phantom (Mutex < Box < dyn Fn (# (# fn_params ,) *) + :: std :: marker :: Send >>) } impl # ig Rfunc # tg # wc { fn call_mut # lg (& mut self , # (# argnames : # argty ,) *) -> std :: result :: Result <# output , &'static str > { match self { Rfunc :: Default (Some (ref mut __mockall_o)) => { :: std :: result :: Result :: Ok (__mockall_o) } , Rfunc :: Default (None) => { Err (# default_err_msg) } , Rfunc :: Mut (ref mut __mockall_f , ref mut __mockall_o) => { * __mockall_o = Some (__mockall_f (# (# argnames ,) *)) ; if let Some (ref mut __mockall_o2) = __mockall_o { :: std :: result :: Result :: Ok (__mockall_o2) } else { unreachable ! () } } , Rfunc :: MutSt (ref mut __mockall_f , ref mut __mockall_o) => { * __mockall_o = Some ((__mockall_f . get_mut ()) (# (# argnames ,) *)) ; if let Some (ref mut __mockall_o2) = __mockall_o { :: std :: result :: Result :: Ok (__mockall_o2) } else { unreachable ! () } } , Rfunc :: Var (ref mut __mockall_o) => { :: std :: result :: Result :: Ok (__mockall_o) } , Rfunc :: _Phantom (_) => unreachable ! () } } } # [allow (single_use_lifetimes)] impl # ig std :: default :: Default for Rfunc # tg # wc { fn default () -> Self { use :: mockall :: ReturnDefault ; Rfunc :: Default (:: mockall :: DefaultReturner ::<# owned_output > :: maybe_return_default ()) } }) . to_tokens (tokens) ; } }
    };
}

impl_34!()