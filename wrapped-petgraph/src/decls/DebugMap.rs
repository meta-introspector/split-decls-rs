macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! DebugMap {
    () => {
        deps!();
        # [doc = " Format the iterator like a map"] pub struct DebugMap < F > (pub F) ;
    };
}

DebugMap!();