macro_rules! deps {
    () => {
        RenameTarget!();
    };
}

macro_rules! RenameRuleExt {
    () => {
        deps!();
        pub trait RenameRuleExt { fn rename (& self , name : impl AsRef < str > , target : RenameTarget) -> String ; }
    };
}

RenameRuleExt!();