macro_rules! deps {
    () => {
        StaticGenericExpectations!();
        GenericExpectations!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl ToTokens for StaticGenericExpectations < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { let argnames = & self . f . argnames ; let argty = & self . f . argty ; let (ig , tg , wc) = self . f . egenerics . split_for_impl () ; let keyid = gen_keyid (& self . f . egenerics) ; let mut any_wc = wc . cloned () ; if self . f . return_ref || self . f . return_refmut { send_syncify (& mut any_wc , self . f . owned_output . clone ()) ; } let tbf = tg . as_turbofish () ; let output = & self . f . output ; let v = & self . f . privmod_vis ; let (call , get , self_ , downcast) = if self . f . return_refmut { (format_ident ! ("call_mut") , format_ident ! ("get_mut") , quote ! (& mut self) , format_ident ! ("downcast_mut")) } else { (format_ident ! ("call") , format_ident ! ("get") , quote ! (& self) , format_ident ! ("downcast_ref")) } ; let (desc_fmt , desc_args) = self . f . desc () ; let no_match_msg = quote ! (concat ! (# desc_fmt , ": No matching expectation found") , # desc_args) ; quote ! (impl # ig :: mockall :: AnyExpectations for Expectations # tg # any_wc { } impl GenericExpectations { # [doc = " Simulating calling the real method."] # v fn # call # ig (# self_ , # (# argnames : # argty ,) *) -> # output # wc { use :: mockall :: { ViaDebug , ViaNothing } ; let __mockall_e = self . store .# get (&:: mockall :: Key :: new ::# keyid ()) . unwrap_or_else (|| panic ! (# no_match_msg)) ; __mockall_e .# downcast ::< Expectations # tg > () . unwrap () .# call (# (# argnames ,) *) } # [doc = " Create a new Expectation."] # v fn expect # ig (& mut self) -> & mut Expectation # tg # any_wc { self . store . entry (:: mockall :: Key :: new ::# keyid ()) . or_insert_with (|| Box :: new (Expectations # tbf :: new ())) . downcast_mut ::< Expectations # tg > () . unwrap () . expect () } }) . to_tokens (tokens) } }
    };
}

impl_52!();