macro_rules! deps {
    () => {
        StaticRfunc!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl ToTokens for StaticRfunc < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let argnames = & self . f . argnames ; let argty = & self . f . argty ; let fn_params = & self . f . fn_params ; let (ig , tg , wc) = self . f . egenerics . split_for_impl () ; let hrtb = self . f . hrtb () ; let lg = lifetimes_to_generics (& self . f . alifetimes) ; let output = & self . f . output ; quote ! (# [allow (clippy :: unused_unit)] enum Rfunc # ig # wc { Default , Expired , Mut (Box < dyn # hrtb FnMut (# (# argty ,) *) -> # output + :: std :: marker :: Send >) , MutSt (:: mockall :: Fragile < Box < dyn # hrtb FnMut (# (# argty ,) *) -> # output >>) , Once (Box < dyn # hrtb FnOnce (# (# argty ,) *) -> # output + :: std :: marker :: Send >) , OnceSt (:: mockall :: Fragile < Box < dyn # hrtb FnOnce (# (# argty ,) *) -> # output >>) , _Phantom (Box < dyn Fn (# (# fn_params ,) *) + :: std :: marker :: Send >) } impl # ig Rfunc # tg # wc { fn call_mut # lg (& mut self , # (# argnames : # argty ,) *) -> std :: result :: Result <# output , &'static str > { match self { Rfunc :: Default => { use :: mockall :: ReturnDefault ; :: mockall :: DefaultReturner ::<# output > :: return_default () } , Rfunc :: Expired => { Err ("called twice, but it returns by move") } , Rfunc :: Mut (__mockall_f) => { :: std :: result :: Result :: Ok (__mockall_f (# (# argnames ,) *)) } , Rfunc :: MutSt (__mockall_f) => { :: std :: result :: Result :: Ok ((__mockall_f . get_mut ()) (# (# argnames ,) *)) } , Rfunc :: Once (_) => { if let Rfunc :: Once (mut __mockall_f) = mem :: replace (self , Rfunc :: Expired) { :: std :: result :: Result :: Ok (__mockall_f (# (# argnames ,) *)) } else { unreachable ! () } } , Rfunc :: OnceSt (_) => { if let Rfunc :: OnceSt (mut __mockall_f) = mem :: replace (self , Rfunc :: Expired) { :: std :: result :: Result :: Ok ((__mockall_f . into_inner ()) (# (# argnames ,) *)) } else { unreachable ! () } } , Rfunc :: _Phantom (_) => unreachable ! () } } } # [allow (single_use_lifetimes)] impl # ig std :: default :: Default for Rfunc # tg # wc { fn default () -> Self { Rfunc :: Default } }) . to_tokens (tokens) ; } }
    };
}

impl_36!()