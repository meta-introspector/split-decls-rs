macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! function {
    () => {
        deps!();
        pub (crate) mod function { use encoding_rs :: EncoderResult ; use super :: Error ; # [doc = " Encode `src_utf8`, which is assumed to be UTF-8 encoded, according to `worktree_encoding` for placement in the working directory,"] # [doc = " and write it to `buf`, possibly resizing it."] # [doc = " Note that the encoding is always applied, there is no conditional even if `worktree_encoding` and the `src` encoding are the same."] pub fn encode_to_worktree (src_utf8 : & [u8] , worktree_encoding : & 'static encoding_rs :: Encoding , buf : & mut Vec < u8 > ,) -> Result < () , Error > { let mut encoder = worktree_encoding . new_encoder () ; let buf_len = encoder . max_buffer_length_from_utf8_if_no_unmappables (src_utf8 . len ()) . ok_or (Error :: Overflow { input_len : src_utf8 . len () , }) ? ; buf . clear () ; buf . resize (buf_len , 0) ; let src = std :: str :: from_utf8 (src_utf8) ? ; let (res , read , written) = encoder . encode_from_utf8_without_replacement (src , buf , true) ; match res { EncoderResult :: InputEmpty => { assert ! (buf_len >= written , "encoding_rs estimates the maximum amount of bytes written correctly") ; assert_eq ! (read , src_utf8 . len () , "input buffer should be fully consumed") ; buf . truncate (written) ; } EncoderResult :: OutputFull => { unreachable ! ("we assure that the output buffer is big enough as per the encoder's estimate") } EncoderResult :: Unmappable (c) => { return Err (Error :: Unmappable { worktree_encoding : worktree_encoding . name () , character : c , }) } } Ok (()) } }
    };
}

function!();