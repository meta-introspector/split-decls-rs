macro_rules! deps {
    () => {
        Clone!();
        Entity!();
    };
}

macro_rules! Personas {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub (crate) struct Personas { user : Entity , committer : Entity , author : Entity , }
    };
}

Personas!();