macro_rules! deps {
    () => {
        Level!();
        Message!();
        Id!();
        MessageOrTitle!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl MessageOrTitle for Message < '_ > { fn level (& self) -> & Level < '_ > { & self . level } fn id (& self) -> Option < & Id < '_ > > { None } fn text (& self) -> & str { self . text . as_ref () } fn allows_styling (& self) -> bool { true } }
    };
}

impl_34!();