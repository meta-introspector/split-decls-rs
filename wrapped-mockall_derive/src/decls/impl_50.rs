macro_rules! deps {
    () => {
        GenericExpectations!();
        StaticGenericExpectations!();
        HashMap!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl ToTokens for GenericExpectations < '_ > { fn to_tokens (& self , tokens : & mut TokenStream) { if ! self . f . is_expectation_generic () { return ; } if ! self . f . is_static () && ! self . f . is_method_generic () { return ; } let ge = StaticGenericExpectations { f : self . f } ; let v = & self . f . privmod_vis ; quote ! (# [doc = " A collection of [`Expectation`](struct.Expectations.html)"] # [doc = " objects for a generic method.  Users will rarely if ever use"] # [doc = " this struct directly."] # [doc (hidden)] # [derive (Default)] # v struct GenericExpectations { store : std :: collections :: hash_map :: HashMap <:: mockall :: Key , Box < dyn :: mockall :: AnyExpectations >> } impl GenericExpectations { # [doc = " Verify that all current expectations are satisfied and clear"] # [doc = " them.  This applies to all sets of generic parameters!"] # v fn checkpoint (& mut self) -> std :: collections :: hash_map :: Drain <:: mockall :: Key , Box < dyn :: mockall :: AnyExpectations >> { self . store . drain () } # v fn new () -> Self { Self :: default () } } # ge) . to_tokens (tokens) ; } }
    };
}

impl_50!()