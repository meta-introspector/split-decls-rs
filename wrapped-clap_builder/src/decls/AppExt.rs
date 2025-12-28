macro_rules! deps {
    () => {
        Extension!();
    };
}

macro_rules! AppExt {
    () => {
        deps!();
        # [allow (dead_code)] pub (crate) trait AppExt : Extension { }
    };
}

AppExt!();