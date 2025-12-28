macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! RAW {
    () => {
        deps!();
        # [doc = " E.g. `1660874655 +0800`"] pub const RAW : Format = Format :: Raw ;
    };
}

RAW!()