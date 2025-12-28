macro_rules! deps {
    () => {
        GenericExpectationGuard!();
        ExpectationGuardCommonMethods!();
        GenericExpectations!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl ToTokens for GenericExpectationGuard < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { if ! self . f . is_static { return ; } let common_methods = ExpectationGuardCommonMethods { f : self . f } ; let (_ , tg , _) = self . f . egenerics . split_for_impl () ; let keyid = gen_keyid (& self . f . egenerics) ; let ltdef = LifetimeParam :: new (Lifetime :: new ("'__mockall_lt" , Span :: call_site ())) ; let mut egenerics = self . f . egenerics . clone () ; egenerics . lt_token . get_or_insert (< Token ! [<] > :: default ()) ; egenerics . params . push (GenericParam :: Lifetime (ltdef)) ; egenerics . gt_token . get_or_insert (< Token ! [>] > :: default ()) ; let (e_ig , e_tg , e_wc) = egenerics . split_for_impl () ; let fn_params = & self . f . fn_params ; let tbf = tg . as_turbofish () ; let v = & self . f . privmod_vis ; quote ! (# [doc (hidden)] # v fn get_expectations () -> &'static :: std :: sync :: Mutex < GenericExpectations > { static CELL : :: std :: sync :: OnceLock <:: std :: sync :: Mutex < GenericExpectations >> = :: std :: sync :: OnceLock :: new () ; CELL . get_or_init (|| :: std :: sync :: Mutex :: new (GenericExpectations :: new ())) } # [doc = " Like an [`&Expectation`](struct.Expectation.html) but"] # [doc = " protected by a Mutex guard.  Useful for mocking static"] # [doc = " methods.  Forwards accesses to an `Expectation` object."] # v struct ExpectationGuard # e_ig # e_wc { guard : MutexGuard <'__mockall_lt , GenericExpectations >, i : usize , _phantom : :: std :: marker :: PhantomData < (# (# fn_params ,) *) >, } # [allow (clippy :: unused_unit)] impl # e_ig ExpectationGuard # e_tg # e_wc { # [doc (hidden)] # v fn new (mut __mockall_guard : MutexGuard <'__mockall_lt , GenericExpectations >) -> Self { let __mockall_ee : & mut Expectations # tg = __mockall_guard . store . entry (:: mockall :: Key :: new ::# keyid ()) . or_insert_with (|| Box :: new (Expectations # tbf :: new ())) . downcast_mut () . unwrap () ; __mockall_ee . expect () ; let __mockall_i = __mockall_ee . 0 . len () - 1 ; ExpectationGuard { guard : __mockall_guard , i : __mockall_i , _phantom : :: std :: marker :: PhantomData } } # common_methods }) . to_tokens (tokens) ; } }
    };
}

impl_26!();