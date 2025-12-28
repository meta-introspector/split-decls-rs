macro_rules! deps {
    () => {
        BigEndian!();
    };
}

macro_rules! BE {
    () => {
        deps!();
        # [doc = " A type alias for [`BigEndian`]."] # [doc = ""] # [doc = " [`BigEndian`]: enum.BigEndian.html"] pub type BE = BigEndian ;
    };
}

BE!()