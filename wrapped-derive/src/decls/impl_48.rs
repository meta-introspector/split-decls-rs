macro_rules! deps {
    () => {
        RenameRuleExt!();
        RenameRule!();
        RenameTarget!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl RenameRuleExt for Option < RenameRule > { fn rename (& self , name : impl AsRef < str > , target : RenameTarget) -> String { self . unwrap_or (target . rule ()) . rename (name) } }
    };
}

impl_48!();