macro_rules! deps {
    () => {
        Conflict!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl Conflict { # [doc = " The amount of conflict marker characters to print by default."] pub const DEFAULT_MARKER_SIZE : u8 = 7 ; # [doc = " The amount of conflict markers to print if this instance contains them, or `None` otherwise"] pub fn marker_size (& self) -> Option < u8 > { match self { Conflict :: Keep { marker_size , .. } => Some (marker_size . get ()) , Conflict :: ResolveWithOurs | Conflict :: ResolveWithTheirs | Conflict :: ResolveWithUnion => None , } } }
    };
}

impl_10!();