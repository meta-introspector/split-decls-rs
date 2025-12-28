macro_rules! deps {
    () => {
        RegisterRuleMap!();
        UnwindContextStorage!();
        ReaderOffset!();
        Register!();
        RegisterRule!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < 'a , R , S > FromIterator < & 'a (Register , RegisterRule < R >) > for RegisterRuleMap < R , S > where R : 'a + ReaderOffset , S : UnwindContextStorage < R > , { fn from_iter < T > (iter : T) -> Self where T : IntoIterator < Item = & 'a (Register , RegisterRule < R >) > , { let iter = iter . into_iter () ; let mut rules = RegisterRuleMap :: default () ; for & (reg , ref rule) in iter . filter (| r | r . 1 . is_defined ()) { rules . set (reg , rule . clone ()) . expect ("This is only used in tests, impl isn't exposed publicly.
                         If you trip this, fix your test" ,) ; } rules } }
    };
}

impl_226!()