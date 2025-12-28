macro_rules! deps {
    () => {
        Read!();
        Write!();
        Io!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < T : Read + Write + Unpin + 'static > Io for T { }
    };
}

impl_226!();