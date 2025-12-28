macro_rules! deps {
    () => {
        RegisterRuleMap!();
        ReaderOffset!();
        UnwindContextStorage!();
    };
}

macro_rules! impl_227 {
    () => {
        deps!();
        impl < T , S > PartialEq for RegisterRuleMap < T , S > where T : ReaderOffset + PartialEq , S : UnwindContextStorage < T > , { fn eq (& self , rhs : & Self) -> bool { for & (reg , ref rule) in & * self . rules { debug_assert ! (rule . is_defined ()) ; if * rule != rhs . get (reg) { return false ; } } for & (reg , ref rhs_rule) in & * rhs . rules { debug_assert ! (rhs_rule . is_defined ()) ; if * rhs_rule != self . get (reg) { return false ; } } true } }
    };
}

impl_227!()