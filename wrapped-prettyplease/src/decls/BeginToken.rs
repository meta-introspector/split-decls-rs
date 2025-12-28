macro_rules! deps {
    () => {
        Breaks!();
    };
}

macro_rules! BeginToken {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub struct BeginToken { pub offset : isize , pub breaks : Breaks , }
    };
}

BeginToken!();