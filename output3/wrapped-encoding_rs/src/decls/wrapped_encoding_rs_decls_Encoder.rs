use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// A converter that encodes a Unicode stream into bytes according to a
/// character encoding in a streaming (incremental) manner.
///
/// The various `encode_*` methods take an input buffer (`src`) and an output
/// buffer `dst` both of which are caller-allocated. There are variants for
/// both UTF-8 and UTF-16 input buffers.
///
/// An `encode_*` method encode characters from `src` into bytes characters
/// stored into `dst` until one of the following three things happens:
///
/// 1. An unmappable character is encountered (`*_without_replacement` variants
///    only).
///
/// 2. The output buffer has been filled so near capacity that the decoder
///    cannot be sure that processing an additional character of input wouldn't
///    cause so much output that the output buffer would overflow.
///
/// 3. All the input characters have been processed.
///
/// The `encode_*` method then returns tuple of a status indicating which one
/// of the three reasons to return happened, how many input code units (`u8`
/// when encoding from UTF-8 and `u16` when encoding from UTF-16) were read,
/// how many output bytes were written (except when encoding into `Vec<u8>`,
/// whose length change indicates this), and in the case of the variants that
/// perform replacement, a boolean indicating whether an unmappable
/// character was replaced with a numeric character reference during the call.
///
/// The number of bytes "written" is what's logically written. Garbage may be
/// written in the output buffer beyond the point logically written to.
///
/// In the case of the methods whose name ends with
/// `*_without_replacement`, the status is an [`EncoderResult`][1] enumeration
/// (possibilities `Unmappable`, `OutputFull` and `InputEmpty` corresponding to
/// the three cases listed above).
///
/// In the case of methods whose name does not end with
/// `*_without_replacement`, unmappable characters are automatically replaced
/// with the corresponding numeric character references and unmappable
/// characters do not cause the methods to return early.
///
/// When encoding from UTF-8 without replacement, the methods are guaranteed
/// not to return indicating that more output space is needed if the length
/// of the output buffer is at least the length returned by
/// [`max_buffer_length_from_utf8_without_replacement()`][2]. When encoding from
/// UTF-8 with replacement, the length of the output buffer that guarantees the
/// methods not to return indicating that more output space is needed in the
/// absence of unmappable characters is given by
/// [`max_buffer_length_from_utf8_if_no_unmappables()`][3]. When encoding from
/// UTF-16 without replacement, the methods are guaranteed not to return
/// indicating that more output space is needed if the length of the output
/// buffer is at least the length returned by
/// [`max_buffer_length_from_utf16_without_replacement()`][4]. When encoding
/// from UTF-16 with replacement, the the length of the output buffer that
/// guarantees the methods not to return indicating that more output space is
/// needed in the absence of unmappable characters is given by
/// [`max_buffer_length_from_utf16_if_no_unmappables()`][5].
/// When encoding with replacement, applications are not expected to size the
/// buffer for the worst case ahead of time but to resize the buffer if there
/// are unmappable characters. This is why max length queries are only available
/// for the case where there are no unmappable characters.
///
/// When encoding from UTF-8, each `src` buffer _must_ be valid UTF-8. (When
/// calling from Rust, the type system takes care of this.) When encoding from
/// UTF-16, unpaired surrogates in the input are treated as U+FFFD REPLACEMENT
/// CHARACTERS. Therefore, in order for astral characters not to turn into a
/// pair of REPLACEMENT CHARACTERS, the caller must ensure that surrogate pairs
/// are not split across input buffer boundaries.
///
/// After an `encode_*` call returns, the output produced so far, taken as a
/// whole from the start of the stream, is guaranteed to consist of a valid
/// byte sequence in the target encoding. (I.e. the code unit sequence for a
/// character is guaranteed not to be split across output buffers. However, due
/// to the stateful nature of ISO-2022-JP, the stream needs to be considered
/// from the start for it to be valid. For other encodings, the validity holds
/// on a per-output buffer basis.)
///
/// The boolean argument `last` indicates that the end of the stream is reached
/// when all the characters in `src` have been consumed. This argument is needed
/// for ISO-2022-JP and is ignored for other encodings.
///
/// An `Encoder` object can be used to incrementally encode a byte stream.
///
/// During the processing of a single stream, the caller must call `encode_*`
/// zero or more times with `last` set to `false` and then call `encode_*` at
/// least once with `last` set to `true`. If `encode_*` returns `InputEmpty`,
/// the processing of the stream has ended. Otherwise, the caller must call
/// `encode_*` again with `last` set to `true` (or treat an `Unmappable` result
/// as a fatal error).
///
/// Once the stream has ended, the `Encoder` object must not be used anymore.
/// That is, you need to create another one to process another stream.
///
/// When the encoder returns `OutputFull` or the encoder returns `Unmappable`
/// and the caller does not wish to treat it as a fatal error, the input buffer
/// `src` may not have been completely consumed. In that case, the caller must
/// pass the unconsumed contents of `src` to `encode_*` again upon the next
/// call.
///
/// [1]: enum.EncoderResult.html
/// [2]: #method.max_buffer_length_from_utf8_without_replacement
/// [3]: #method.max_buffer_length_from_utf8_if_no_unmappables
/// [4]: #method.max_buffer_length_from_utf16_without_replacement
/// [5]: #method.max_buffer_length_from_utf16_if_no_unmappables
///
/// # Infinite loops
///
/// When converting with a fixed-size output buffer whose size is too small to
/// accommodate one character of output, an infinite loop ensues. When
/// converting with a fixed-size output buffer, it generally makes sense to
/// make the buffer fairly large (e.g. couple of kilobytes).
pub struct Encoder {
    encoding: &'static Encoding,
    variant: VariantEncoder,
}
