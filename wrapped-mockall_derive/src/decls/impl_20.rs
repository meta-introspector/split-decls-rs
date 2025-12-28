macro_rules! deps {
    () => {
        CommonExpectationsMethods!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl ToTokens for CommonExpectationsMethods < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let (ig , tg , wc) = self . f . egenerics . split_for_impl () ; let v = & self . f . privmod_vis ; quote ! (# [doc = " A collection of [`Expectation`](struct.Expectations.html)"] # [doc = " objects.  Users will rarely if ever use this struct directly."] # [doc (hidden)] # v struct Expectations # ig (Vec < Expectation # tg >) # wc ; impl # ig Expectations # tg # wc { # [doc = " Verify that all current expectations are satisfied and clear"] # [doc = " them."] # v fn checkpoint (& mut self) -> std :: vec :: Drain < Expectation # tg > { self . 0 . drain (..) } # [doc = " Create a new expectation for this method."] # v fn expect (& mut self) -> & mut Expectation # tg { self . 0 . push (Expectation :: default ()) ; let __mockall_l = self . 0 . len () ; & mut self . 0 [__mockall_l - 1] } # v const fn new () -> Self { Self (Vec :: new ()) } } # [allow (single_use_lifetimes)] impl # ig Default for Expectations # tg # wc { fn default () -> Self { Expectations :: new () } }) . to_tokens (tokens) ; } }
    };
}

impl_20!();