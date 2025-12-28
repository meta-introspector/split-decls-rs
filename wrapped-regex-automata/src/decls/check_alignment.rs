macro_rules! deps {
    () => {
        DeserializeError!();
    };
}

macro_rules! check_alignment {
    () => {
        deps!();
        # [doc = " Checks that the given slice has an alignment that matches `T`."] # [doc = ""] # [doc = " This is useful for checking that a slice has an appropriate alignment"] # [doc = " before casting it to a &[T]. Note though that alignment is not itself"] # [doc = " sufficient to perform the cast for any `T`."] pub (crate) fn check_alignment < T > (slice : & [u8] ,) -> Result < () , DeserializeError > { let alignment = core :: mem :: align_of :: < T > () ; let address = slice . as_ptr () . as_usize () ; if address % alignment == 0 { return Ok (()) ; } Err (DeserializeError :: alignment_mismatch (alignment , address)) }
    };
}

check_alignment!();