macro_rules! deps {
    () => {
        ByteSize!();
    };
}

macro_rules! Format {
    () => {
        deps!();
        # [doc = " Format / style to use when displaying a [`ByteSize`]."] # [derive (Debug , Clone , Copy)] pub (crate) enum Format { Iec , IecShort , Si , SiShort , }
    };
}

Format!()