// Generated macro for check_alignment (function)
macro_rules! Depcrate_util_wirecheck_alignment {
() => {
// Module: crate::util::wire
// Provides: {"check_alignment"}
// Dependencies: {}
# [doc = " Checks that the given slice has an alignment that matches `T`."] # [doc = ""] # [doc = " This is useful for checking that a slice has an appropriate alignment"] # [doc = " before casting it to a &[T]. Note though that alignment is not itself"] # [doc = " sufficient to perform the cast for any `T`."] pub (crate) fn check_alignment < T > (slice : & [u8] ,) -> Result < () , DeserializeError > { let alignment = core :: mem :: align_of :: < T > () ; let address = slice . as_ptr () . as_usize () ; if address % alignment == 0 { return Ok (()) ; } Err (DeserializeError :: alignment_mismatch (alignment , address)) }
};
}
