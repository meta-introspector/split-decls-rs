macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! MemberUnraw {
    () => {
        deps!();
        # [derive (Clone)] pub enum MemberUnraw { Named (IdentUnraw) , Unnamed (Index) , }
    };
}

MemberUnraw!()