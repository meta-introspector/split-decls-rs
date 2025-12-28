macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! check_slice_len {
    () => {
        deps!();
        # [doc = " Checks that the given slice has some minimal length. If it's smaller than"] # [doc = " the bound given, then a \"buffer too small\" error is returned with `what`"] # [doc = " describing what the buffer represents."] pub (crate) fn check_slice_len < T > (slice : & [T] , at_least_len : usize , what : & 'static str ,) -> Result < () , DeserializeError > { if slice . len () < at_least_len { return Err (DeserializeError :: buffer_too_small (what)) ; } Ok (()) }
    };
}

check_slice_len!();