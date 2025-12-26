use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A converter that decodes a byte stream into Unicode according to a
/// character encoding in a streaming (incremental) manner.
///
/// The various `decode_*` methods take an input buffer (`src`) and an output
/// buffer `dst` both of which are caller-allocated. There are variants for
/// both UTF-8 and UTF-16 output buffers.
///
/// A `decode_*` method decodes bytes from `src` into Unicode characters stored
/// into `dst` until one of the following three things happens:
///
/// 1. A malformed byte sequence is encountered (`*_without_replacement`
///    variants only).
///
/// 2. The output buffer has been filled so near capacity that the decoder
///    cannot be sure that processing an additional byte of input wouldn't
///    cause so much output that the output buffer would overflow.
///
/// 3. All the input bytes have been processed.
///
/// The `decode_*` method then returns tuple of a status indicating which one
/// of the three reasons to return happened, how many input bytes were read,
/// how many output code units (`u8` when decoding into UTF-8 and `u16`
/// when decoding to UTF-16) were written (except when decoding into `String`,
/// whose length change indicates this), and in the case of the
/// variants performing replacement, a boolean indicating whether an error was
/// replaced with the REPLACEMENT CHARACTER during the call.
///
/// The number of bytes "written" is what's logically written. Garbage may be
/// written in the output buffer beyond the point logically written to.
/// Therefore, if you wish to decode into an `&mut str`, you should use the
/// methods that take an `&mut str` argument instead of the ones that take an
/// `&mut [u8]` argument. The former take care of overwriting the trailing
/// garbage to ensure the UTF-8 validity of the `&mut str` as a whole, but the
/// latter don't.
///
/// In the case of the `*_without_replacement` variants, the status is a
/// [`DecoderResult`][1] enumeration (possibilities `Malformed`, `OutputFull` and
/// `InputEmpty` corresponding to the three cases listed above).
///
/// In the case of methods whose name does not end with
/// `*_without_replacement`, malformed sequences are automatically replaced
/// with the REPLACEMENT CHARACTER and errors do not cause the methods to
/// return early.
///
/// When decoding to UTF-8, the output buffer must have at least 4 bytes of
/// space. When decoding to UTF-16, the output buffer must have at least two
/// UTF-16 code units (`u16`) of space.
///
/// When decoding to UTF-8 without replacement, the methods are guaranteed
/// not to return indicating that more output space is needed if the length
/// of the output buffer is at least the length returned by
/// [`max_utf8_buffer_length_without_replacement()`][2]. When decoding to UTF-8
/// with replacement, the length of the output buffer that guarantees the
/// methods not to return indicating that more output space is needed is given
/// by [`max_utf8_buffer_length()`][3]. When decoding to UTF-16 with
/// or without replacement, the length of the output buffer that guarantees
/// the methods not to return indicating that more output space is needed is
/// given by [`max_utf16_buffer_length()`][4].
///
/// The output written into `dst` is guaranteed to be valid UTF-8 or UTF-16,
/// and the output after each `decode_*` call is guaranteed to consist of
/// complete characters. (I.e. the code unit sequence for the last character is
/// guaranteed not to be split across output buffers.)
///
/// The boolean argument `last` indicates that the end of the stream is reached
/// when all the bytes in `src` have been consumed.
///
/// A `Decoder` object can be used to incrementally decode a byte stream.
///
/// During the processing of a single stream, the caller must call `decode_*`
/// zero or more times with `last` set to `false` and then call `decode_*` at
/// least once with `last` set to `true`. If `decode_*` returns `InputEmpty`,
/// the processing of the stream has ended. Otherwise, the caller must call
/// `decode_*` again with `last` set to `true` (or treat a `Malformed` result as
///  a fatal error).
///
/// Once the stream has ended, the `Decoder` object must not be used anymore.
/// That is, you need to create another one to process another stream.
///
/// When the decoder returns `OutputFull` or the decoder returns `Malformed` and
/// the caller does not wish to treat it as a fatal error, the input buffer
/// `src` may not have been completely consumed. In that case, the caller must
/// pass the unconsumed contents of `src` to `decode_*` again upon the next
/// call.
///
/// [1]: enum.DecoderResult.html
/// [2]: #method.max_utf8_buffer_length_without_replacement
/// [3]: #method.max_utf8_buffer_length
/// [4]: #method.max_utf16_buffer_length
///
/// # Infinite loops
///
/// When converting with a fixed-size output buffer whose size is too small to
/// accommodate one character or (when applicable) one numeric character
/// reference of output, an infinite loop ensues. When converting with a
/// fixed-size output buffer, it generally makes sense to make the buffer
/// fairly large (e.g. couple of kilobytes).
pub struct Decoder {
    encoding: &'static Encoding,
    variant: VariantDecoder,
    life_cycle: DecoderLifeCycle,
}
