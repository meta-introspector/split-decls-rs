macro_rules! deps {
    () => {
        Level!();
        MessageOrTitle!();
        Title!();
        Id!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl MessageOrTitle for Title < '_ > { fn level (& self) -> & Level < '_ > { & self . level } fn id (& self) -> Option < & Id < '_ > > { self . id . as_ref () } fn text (& self) -> & str { self . text . as_ref () } fn allows_styling (& self) -> bool { self . allows_styling } }
    };
}

impl_33!()