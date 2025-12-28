macro_rules! deps {
    () => {
        LittleEndian!();
    };
}

macro_rules! LE {
    () => {
        deps!();
        # [doc = " A type alias for [`LittleEndian`]."] # [doc = ""] # [doc = " [`LittleEndian`]: enum.LittleEndian.html"] pub type LE = LittleEndian ;
    };
}

LE!()