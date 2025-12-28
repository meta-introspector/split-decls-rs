macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! UNIX {
    () => {
        deps!();
        # [doc = " E.g. `123456789`"] pub const UNIX : Format = Format :: Unix ;
    };
}

UNIX!()