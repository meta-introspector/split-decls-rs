macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! DEFAULT {
    () => {
        deps!();
        # [doc = " E.g. `Thu Sep 4 10:45:06 2022 -0400`. This is output by `git log --pretty=%ad`."] pub const DEFAULT : CustomFormat = CustomFormat ("%a %b %-d %H:%M:%S %Y %z") ;
    };
}

DEFAULT!()