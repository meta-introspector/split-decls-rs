macro_rules! deps {
    () => {
        PermitVariants!();
    };
}

macro_rules! LowerTypeRelativePathMode {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] enum LowerTypeRelativePathMode { Type (PermitVariants) , Const , }
    };
}

LowerTypeRelativePathMode!()