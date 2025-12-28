macro_rules! deps {
    () => {
        Error!();
        Commit!();
        CommitRef!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [doc = " Conversion"] impl CommitRef < '_ > { # [doc = " Copy all fields of this instance into a fully owned commit, consuming this instance."] pub fn into_owned (self) -> Result < Commit , crate :: decode :: Error > { self . try_into () } # [doc = " Copy all fields of this instance into a fully owned commit, internally cloning this instance."] pub fn to_owned (self) -> Result < Commit , crate :: decode :: Error > { self . try_into () } }
    };
}

impl_47!();