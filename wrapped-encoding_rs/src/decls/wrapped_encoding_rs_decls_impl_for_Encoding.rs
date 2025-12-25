use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Encoding {
    /// Implements the
    /// [_get an encoding_](https://encoding.spec.whatwg.org/#concept-encoding-get)
    /// algorithm.
    ///
    /// If, after ASCII-lowercasing and removing leading and trailing
    /// whitespace, the argument matches a label defined in the Encoding
    /// Standard, `Some(&'static Encoding)` representing the corresponding
    /// encoding is returned. If there is no match, `None` is returned.
    ///
    /// This is the right method to use if the action upon the method returning
    /// `None` is to use a fallback encoding (e.g. `WINDOWS_1252`) instead.
    /// When the action upon the method returning `None` is not to proceed with
    /// a fallback but to refuse processing, `for_label_no_replacement()` is more
    /// appropriate.
    ///
    /// The argument is of type `&[u8]` instead of `&str` to save callers
    /// that are extracting the label from a non-UTF-8 protocol the trouble
    /// of conversion to UTF-8. (If you have a `&str`, just call `.as_bytes()`
    /// on it.)
    ///
    /// Available via the C wrapper.
    ///
    /// # Example
    /// ```
    /// use encoding_rs::Encoding;
    ///
    /// assert_eq!(Some(encoding_rs::UTF_8), Encoding::for_label(b"utf-8"));
    /// assert_eq!(Some(encoding_rs::UTF_8), Encoding::for_label(b"unicode11utf8"));
    ///
    /// assert_eq!(Some(encoding_rs::ISO_8859_2), Encoding::for_label(b"latin2"));
    ///
    /// assert_eq!(Some(encoding_rs::UTF_16BE), Encoding::for_label(b"utf-16be"));
    ///
    /// assert_eq!(None, Encoding::for_label(b"unrecognized label"));
    /// ```
    pub fn for_label(label: &[u8]) -> Option<&'static Encoding> {
        let mut trimmed = [0u8; LONGEST_LABEL_LENGTH];
        let mut trimmed_pos = 0usize;
        let mut iter = label.into_iter();
        loop {
            match iter.next() {
                None => {
                    return None;
                }
                Some(byte) => match *byte {
                    0x09u8 | 0x0Au8 | 0x0Cu8 | 0x0Du8 | 0x20u8 => {
                        continue;
                    }
                    b'A'..=b'Z' => {
                        trimmed[trimmed_pos] = *byte + 0x20u8;
                        trimmed_pos = 1usize;
                        break;
                    }
                    b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b':' | b'.' => {
                        trimmed[trimmed_pos] = *byte;
                        trimmed_pos = 1usize;
                        break;
                    }
                    _ => {
                        return None;
                    }
                },
            }
        }
        loop {
            match iter.next() {
                None => {
                    break;
                }
                Some(byte) => match *byte {
                    0x09u8 | 0x0Au8 | 0x0Cu8 | 0x0Du8 | 0x20u8 => {
                        break;
                    }
                    b'A'..=b'Z' => {
                        if trimmed_pos == LONGEST_LABEL_LENGTH {
                            return None;
                        }
                        trimmed[trimmed_pos] = *byte + 0x20u8;
                        trimmed_pos += 1usize;
                        continue;
                    }
                    b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b':' | b'.' => {
                        if trimmed_pos == LONGEST_LABEL_LENGTH {
                            return None;
                        }
                        trimmed[trimmed_pos] = *byte;
                        trimmed_pos += 1usize;
                        continue;
                    }
                    _ => {
                        return None;
                    }
                },
            }
        }
        loop {
            match iter.next() {
                None => {
                    break;
                }
                Some(byte) => match *byte {
                    0x09u8 | 0x0Au8 | 0x0Cu8 | 0x0Du8 | 0x20u8 => {
                        continue;
                    }
                    _ => {
                        return None;
                    }
                },
            }
        }
        let candidate = &trimmed[..trimmed_pos];
        match LABELS_SORTED.binary_search_by(|probe| {
            let bytes = probe.as_bytes();
            let c = bytes.len().cmp(&candidate.len());
            if c != Ordering::Equal {
                return c;
            }
            let probe_iter = bytes.iter().rev();
            let candidate_iter = candidate.iter().rev();
            probe_iter.cmp(candidate_iter)
        }) {
            Ok(i) => Some(ENCODINGS_IN_LABEL_SORT[i]),
            Err(_) => None,
        }
    }
    /// This method behaves the same as `for_label()`, except when `for_label()`
    /// would return `Some(REPLACEMENT)`, this method returns `None` instead.
    ///
    /// This method is useful in scenarios where a fatal error is required
    /// upon invalid label, because in those cases the caller typically wishes
    /// to treat the labels that map to the replacement encoding as fatal
    /// errors, too.
    ///
    /// It is not OK to use this method when the action upon the method returning
    /// `None` is to use a fallback encoding (e.g. `WINDOWS_1252`). In such a
    /// case, the `for_label()` method should be used instead in order to avoid
    /// unsafe fallback for labels that `for_label()` maps to `Some(REPLACEMENT)`.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn for_label_no_replacement(label: &[u8]) -> Option<&'static Encoding> {
        match Encoding::for_label(label) {
            None => None,
            Some(encoding) => {
                if encoding == REPLACEMENT {
                    None
                } else {
                    Some(encoding)
                }
            }
        }
    }
    /// Performs non-incremental BOM sniffing.
    ///
    /// The argument must either be a buffer representing the entire input
    /// stream (non-streaming case) or a buffer representing at least the first
    /// three bytes of the input stream (streaming case).
    ///
    /// Returns `Some((UTF_8, 3))`, `Some((UTF_16LE, 2))` or
    /// `Some((UTF_16BE, 2))` if the argument starts with the UTF-8, UTF-16LE
    /// or UTF-16BE BOM or `None` otherwise.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn for_bom(buffer: &[u8]) -> Option<(&'static Encoding, usize)> {
        if buffer.starts_with(b"\xEF\xBB\xBF") {
            Some((UTF_8, 3))
        } else if buffer.starts_with(b"\xFF\xFE") {
            Some((UTF_16LE, 2))
        } else if buffer.starts_with(b"\xFE\xFF") {
            Some((UTF_16BE, 2))
        } else {
            None
        }
    }
    /// Returns the name of this encoding.
    ///
    /// This name is appropriate to return as-is from the DOM
    /// `document.characterSet` property.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn name(&'static self) -> &'static str {
        self.name
    }
    /// Checks whether the _output encoding_ of this encoding can encode every
    /// `char`. (Only true if the output encoding is UTF-8.)
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn can_encode_everything(&'static self) -> bool {
        self.output_encoding() == UTF_8
    }
    /// Checks whether the bytes 0x00...0x7F map exclusively to the characters
    /// U+0000...U+007F and vice versa.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn is_ascii_compatible(&'static self) -> bool {
        !(self == REPLACEMENT || self == UTF_16BE || self == UTF_16LE || self == ISO_2022_JP)
    }
    /// Checks whether this encoding maps one byte to one Basic Multilingual
    /// Plane code point (i.e. byte length equals decoded UTF-16 length) and
    /// vice versa (for mappable characters).
    ///
    /// `true` iff this encoding is on the list of [Legacy single-byte
    /// encodings](https://encoding.spec.whatwg.org/#legacy-single-byte-encodings)
    /// in the spec or x-user-defined.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn is_single_byte(&'static self) -> bool {
        self.variant.is_single_byte()
    }
    /// Checks whether the bytes 0x00...0x7F map mostly to the characters
    /// U+0000...U+007F and vice versa.
    #[cfg(feature = "alloc")]
    #[inline]
    fn is_potentially_borrowable(&'static self) -> bool {
        !(self == REPLACEMENT || self == UTF_16BE || self == UTF_16LE)
    }
    /// Returns the _output encoding_ of this encoding. This is UTF-8 for
    /// UTF-16BE, UTF-16LE, and replacement and the encoding itself otherwise.
    ///
    /// _Note:_ The _output encoding_ concept is needed for form submission and
    /// error handling in the query strings of URLs in the Web Platform.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn output_encoding(&'static self) -> &'static Encoding {
        if self == REPLACEMENT || self == UTF_16BE || self == UTF_16LE {
            UTF_8
        } else {
            self
        }
    }
    /// Decode complete input to `Cow<'a, str>` _with BOM sniffing_ and with
    /// malformed sequences replaced with the REPLACEMENT CHARACTER when the
    /// entire input is available as a single buffer (i.e. the end of the
    /// buffer marks the end of the stream).
    ///
    /// The BOM, if any, does not appear in the output.
    ///
    /// This method implements the (non-streaming version of) the
    /// [_decode_](https://encoding.spec.whatwg.org/#decode) spec concept.
    ///
    /// The second item in the returned tuple is the encoding that was actually
    /// used (which may differ from this encoding thanks to BOM sniffing).
    ///
    /// The third item in the returned tuple indicates whether there were
    /// malformed sequences (that were replaced with the REPLACEMENT CHARACTER).
    ///
    /// _Note:_ It is wrong to use this when the input buffer represents only
    /// a segment of the input instead of the whole input. Use `new_decoder()`
    /// when decoding segmented input.
    ///
    /// This method performs a one or two heap allocations for the backing
    /// buffer of the `String` when unable to borrow. (One allocation if not
    /// errors and potentially another one in the presence of errors.) The
    /// first allocation assumes jemalloc and may not be optimal with
    /// allocators that do not use power-of-two buckets. A borrow is performed
    /// if decoding UTF-8 and the input is valid UTF-8, if decoding an
    /// ASCII-compatible encoding and the input is ASCII-only, or when decoding
    /// ISO-2022-JP and the input is entirely in the ASCII state without state
    /// transitions.
    ///
    /// # Panics
    ///
    /// If the size calculation for a heap-allocated backing buffer overflows
    /// `usize`.
    ///
    /// Available to Rust only and only with the `alloc` feature enabled (enabled
    /// by default).
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn decode<'a>(&'static self, bytes: &'a [u8]) -> (Cow<'a, str>, &'static Encoding, bool) {
        let (encoding, without_bom) = match Encoding::for_bom(bytes) {
            Some((encoding, bom_length)) => (encoding, &bytes[bom_length..]),
            None => (self, bytes),
        };
        let (cow, had_errors) = encoding.decode_without_bom_handling(without_bom);
        (cow, encoding, had_errors)
    }
    /// Decode complete input to `Cow<'a, str>` _with BOM removal_ and with
    /// malformed sequences replaced with the REPLACEMENT CHARACTER when the
    /// entire input is available as a single buffer (i.e. the end of the
    /// buffer marks the end of the stream).
    ///
    /// Only an initial byte sequence that is a BOM for this encoding is removed.
    ///
    /// When invoked on `UTF_8`, this method implements the (non-streaming
    /// version of) the
    /// [_UTF-8 decode_](https://encoding.spec.whatwg.org/#utf-8-decode) spec
    /// concept.
    ///
    /// The second item in the returned pair indicates whether there were
    /// malformed sequences (that were replaced with the REPLACEMENT CHARACTER).
    ///
    /// _Note:_ It is wrong to use this when the input buffer represents only
    /// a segment of the input instead of the whole input. Use
    /// `new_decoder_with_bom_removal()` when decoding segmented input.
    ///
    /// This method performs a one or two heap allocations for the backing
    /// buffer of the `String` when unable to borrow. (One allocation if not
    /// errors and potentially another one in the presence of errors.) The
    /// first allocation assumes jemalloc and may not be optimal with
    /// allocators that do not use power-of-two buckets. A borrow is performed
    /// if decoding UTF-8 and the input is valid UTF-8, if decoding an
    /// ASCII-compatible encoding and the input is ASCII-only, or when decoding
    /// ISO-2022-JP and the input is entirely in the ASCII state without state
    /// transitions.
    ///
    /// # Panics
    ///
    /// If the size calculation for a heap-allocated backing buffer overflows
    /// `usize`.
    ///
    /// Available to Rust only and only with the `alloc` feature enabled (enabled
    /// by default).
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn decode_with_bom_removal<'a>(&'static self, bytes: &'a [u8]) -> (Cow<'a, str>, bool) {
        let without_bom = if self == UTF_8 && bytes.starts_with(b"\xEF\xBB\xBF") {
            &bytes[3..]
        } else if (self == UTF_16LE && bytes.starts_with(b"\xFF\xFE"))
            || (self == UTF_16BE && bytes.starts_with(b"\xFE\xFF"))
        {
            &bytes[2..]
        } else {
            bytes
        };
        self.decode_without_bom_handling(without_bom)
    }
    /// Decode complete input to `Cow<'a, str>` _without BOM handling_ and
    /// with malformed sequences replaced with the REPLACEMENT CHARACTER when
    /// the entire input is available as a single buffer (i.e. the end of the
    /// buffer marks the end of the stream).
    ///
    /// When invoked on `UTF_8`, this method implements the (non-streaming
    /// version of) the
    /// [_UTF-8 decode without BOM_](https://encoding.spec.whatwg.org/#utf-8-decode-without-bom)
    /// spec concept.
    ///
    /// The second item in the returned pair indicates whether there were
    /// malformed sequences (that were replaced with the REPLACEMENT CHARACTER).
    ///
    /// _Note:_ It is wrong to use this when the input buffer represents only
    /// a segment of the input instead of the whole input. Use
    /// `new_decoder_without_bom_handling()` when decoding segmented input.
    ///
    /// This method performs a one or two heap allocations for the backing
    /// buffer of the `String` when unable to borrow. (One allocation if not
    /// errors and potentially another one in the presence of errors.) The
    /// first allocation assumes jemalloc and may not be optimal with
    /// allocators that do not use power-of-two buckets. A borrow is performed
    /// if decoding UTF-8 and the input is valid UTF-8, if decoding an
    /// ASCII-compatible encoding and the input is ASCII-only, or when decoding
    /// ISO-2022-JP and the input is entirely in the ASCII state without state
    /// transitions.
    ///
    /// # Panics
    ///
    /// If the size calculation for a heap-allocated backing buffer overflows
    /// `usize`.
    ///
    /// Available to Rust only and only with the `alloc` feature enabled (enabled
    /// by default).
    #[cfg(feature = "alloc")]
    pub fn decode_without_bom_handling<'a>(&'static self, bytes: &'a [u8]) -> (Cow<'a, str>, bool) {
        let (mut decoder, mut string, mut total_read) = if self.is_potentially_borrowable() {
            let valid_up_to = if self == UTF_8 {
                utf8_valid_up_to(bytes)
            } else if self == ISO_2022_JP {
                iso_2022_jp_ascii_valid_up_to(bytes)
            } else {
                ascii_valid_up_to(bytes)
            };
            if valid_up_to == bytes.len() {
                let str: &str = unsafe { core::str::from_utf8_unchecked(bytes) };
                return (Cow::Borrowed(str), false);
            }
            let decoder = self.new_decoder_without_bom_handling();
            let rounded_without_replacement = checked_next_power_of_two(checked_add(
                valid_up_to,
                decoder.max_utf8_buffer_length_without_replacement(bytes.len() - valid_up_to),
            ));
            let with_replacement = checked_add(
                valid_up_to,
                decoder.max_utf8_buffer_length(bytes.len() - valid_up_to),
            );
            let mut string = String::with_capacity(
                checked_min(rounded_without_replacement, with_replacement).unwrap(),
            );
            unsafe {
                let vec = string.as_mut_vec();
                vec.set_len(valid_up_to);
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), vec.as_mut_ptr(), valid_up_to);
            }
            (decoder, string, valid_up_to)
        } else {
            let decoder = self.new_decoder_without_bom_handling();
            let rounded_without_replacement = checked_next_power_of_two(
                decoder.max_utf8_buffer_length_without_replacement(bytes.len()),
            );
            let with_replacement = decoder.max_utf8_buffer_length(bytes.len());
            let string = String::with_capacity(
                checked_min(rounded_without_replacement, with_replacement).unwrap(),
            );
            (decoder, string, 0)
        };
        let mut total_had_errors = false;
        loop {
            let (result, read, had_errors) =
                decoder.decode_to_string(&bytes[total_read..], &mut string, true);
            total_read += read;
            total_had_errors |= had_errors;
            match result {
                CoderResult::InputEmpty => {
                    debug_assert_eq!(total_read, bytes.len());
                    return (Cow::Owned(string), total_had_errors);
                }
                CoderResult::OutputFull => {
                    let needed = decoder.max_utf8_buffer_length(bytes.len() - total_read);
                    string.reserve(needed.unwrap());
                }
            }
        }
    }
    /// Decode complete input to `Cow<'a, str>` _without BOM handling_ and
    /// _with malformed sequences treated as fatal_ when the entire input is
    /// available as a single buffer (i.e. the end of the buffer marks the end
    /// of the stream).
    ///
    /// When invoked on `UTF_8`, this method implements the (non-streaming
    /// version of) the
    /// [_UTF-8 decode without BOM or fail_](https://encoding.spec.whatwg.org/#utf-8-decode-without-bom-or-fail)
    /// spec concept.
    ///
    /// Returns `None` if a malformed sequence was encountered and the result
    /// of the decode as `Some(String)` otherwise.
    ///
    /// _Note:_ It is wrong to use this when the input buffer represents only
    /// a segment of the input instead of the whole input. Use
    /// `new_decoder_without_bom_handling()` when decoding segmented input.
    ///
    /// This method performs a single heap allocation for the backing
    /// buffer of the `String` when unable to borrow. A borrow is performed if
    /// decoding UTF-8 and the input is valid UTF-8, if decoding an
    /// ASCII-compatible encoding and the input is ASCII-only, or when decoding
    /// ISO-2022-JP and the input is entirely in the ASCII state without state
    /// transitions.
    ///
    /// # Panics
    ///
    /// If the size calculation for a heap-allocated backing buffer overflows
    /// `usize`.
    ///
    /// Available to Rust only and only with the `alloc` feature enabled (enabled
    /// by default).
    #[cfg(feature = "alloc")]
    pub fn decode_without_bom_handling_and_without_replacement<'a>(
        &'static self,
        bytes: &'a [u8],
    ) -> Option<Cow<'a, str>> {
        if self == UTF_8 {
            let valid_up_to = utf8_valid_up_to(bytes);
            if valid_up_to == bytes.len() {
                let str: &str = unsafe { core::str::from_utf8_unchecked(bytes) };
                return Some(Cow::Borrowed(str));
            }
            return None;
        }
        let (mut decoder, mut string, input) = if self.is_potentially_borrowable() {
            let valid_up_to = if self == ISO_2022_JP {
                iso_2022_jp_ascii_valid_up_to(bytes)
            } else {
                ascii_valid_up_to(bytes)
            };
            if valid_up_to == bytes.len() {
                let str: &str = unsafe { core::str::from_utf8_unchecked(bytes) };
                return Some(Cow::Borrowed(str));
            }
            let decoder = self.new_decoder_without_bom_handling();
            let mut string = String::with_capacity(
                checked_add(
                    valid_up_to,
                    decoder.max_utf8_buffer_length_without_replacement(bytes.len() - valid_up_to),
                )
                .unwrap(),
            );
            unsafe {
                let vec = string.as_mut_vec();
                vec.set_len(valid_up_to);
                core::ptr::copy_nonoverlapping(bytes.as_ptr(), vec.as_mut_ptr(), valid_up_to);
            }
            (decoder, string, &bytes[valid_up_to..])
        } else {
            let decoder = self.new_decoder_without_bom_handling();
            let string = String::with_capacity(
                decoder
                    .max_utf8_buffer_length_without_replacement(bytes.len())
                    .unwrap(),
            );
            (decoder, string, bytes)
        };
        let (result, read) = decoder.decode_to_string_without_replacement(input, &mut string, true);
        match result {
            DecoderResult::InputEmpty => {
                debug_assert_eq!(read, input.len());
                Some(Cow::Owned(string))
            }
            DecoderResult::Malformed(_, _) => None,
            DecoderResult::OutputFull => unreachable!(),
        }
    }
    /// Encode complete input to `Cow<'a, [u8]>` using the
    /// [_output encoding_](Encoding::output_encoding) of this encoding with
    /// unmappable characters replaced with decimal numeric character references
    /// when the entire input is available as a single buffer (i.e. the end of
    /// the buffer marks the end of the stream).
    ///
    /// This method implements the (non-streaming version of) the
    /// [_encode_](https://encoding.spec.whatwg.org/#encode) spec concept. For
    /// the [_UTF-8 encode_](https://encoding.spec.whatwg.org/#utf-8-encode)
    /// spec concept, it is slightly more efficient to use
    /// <code><var>string</var>.as_bytes()</code> instead of invoking this
    /// method on `UTF_8`.
    ///
    /// The second item in the returned tuple is the encoding that was actually
    /// used (*which may differ from this encoding thanks to some encodings
    /// having UTF-8 as their output encoding*).
    ///
    /// The third item in the returned tuple indicates whether there were
    /// unmappable characters (that were replaced with HTML numeric character
    /// references).
    ///
    /// _Note:_ It is wrong to use this when the input buffer represents only
    /// a segment of the input instead of the whole input. Use `new_encoder()`
    /// when encoding segmented output.
    ///
    /// When encoding to UTF-8 or when encoding an ASCII-only input to a
    /// ASCII-compatible encoding, this method returns a borrow of the input
    /// without a heap allocation. Otherwise, this method performs a single
    /// heap allocation for the backing buffer of the `Vec<u8>` if there are no
    /// unmappable characters and potentially multiple heap allocations if
    /// there are. These allocations are tuned for jemalloc and may not be
    /// optimal when using a different allocator that doesn't use power-of-two
    /// buckets.
    ///
    /// # Panics
    ///
    /// If the size calculation for a heap-allocated backing buffer overflows
    /// `usize`.
    ///
    /// Available to Rust only and only with the `alloc` feature enabled (enabled
    /// by default).
    #[cfg(feature = "alloc")]
    pub fn encode<'a>(&'static self, string: &'a str) -> (Cow<'a, [u8]>, &'static Encoding, bool) {
        let output_encoding = self.output_encoding();
        if output_encoding == UTF_8 {
            return (Cow::Borrowed(string.as_bytes()), output_encoding, false);
        }
        debug_assert!(output_encoding.is_potentially_borrowable());
        let bytes = string.as_bytes();
        let valid_up_to = if output_encoding == ISO_2022_JP {
            iso_2022_jp_ascii_valid_up_to(bytes)
        } else {
            ascii_valid_up_to(bytes)
        };
        if valid_up_to == bytes.len() {
            return (Cow::Borrowed(bytes), output_encoding, false);
        }
        let mut encoder = output_encoding.new_encoder();
        let mut vec: Vec<u8> = Vec::with_capacity(
            (checked_add(
                valid_up_to,
                encoder.max_buffer_length_from_utf8_if_no_unmappables(string.len() - valid_up_to),
            ))
            .unwrap()
            .next_power_of_two(),
        );
        unsafe {
            vec.set_len(valid_up_to);
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), vec.as_mut_ptr(), valid_up_to);
        }
        let mut total_read = valid_up_to;
        let mut total_had_errors = false;
        loop {
            let (result, read, had_errors) =
                encoder.encode_from_utf8_to_vec(&string[total_read..], &mut vec, true);
            total_read += read;
            total_had_errors |= had_errors;
            match result {
                CoderResult::InputEmpty => {
                    debug_assert_eq!(total_read, string.len());
                    return (Cow::Owned(vec), output_encoding, total_had_errors);
                }
                CoderResult::OutputFull => {
                    let needed = encoder
                        .max_buffer_length_from_utf8_if_no_unmappables(string.len() - total_read);
                    let rounded = (checked_add(vec.capacity(), needed))
                        .unwrap()
                        .next_power_of_two();
                    let additional = rounded - vec.len();
                    vec.reserve_exact(additional);
                }
            }
        }
    }
    fn new_variant_decoder(&'static self) -> VariantDecoder {
        self.variant.new_variant_decoder()
    }
    /// Instantiates a new decoder for this encoding with BOM sniffing enabled.
    ///
    /// BOM sniffing may cause the returned decoder to morph into a decoder
    /// for UTF-8, UTF-16LE or UTF-16BE instead of this encoding. The BOM
    /// does not appear in the output.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn new_decoder(&'static self) -> Decoder {
        Decoder::new(self, self.new_variant_decoder(), BomHandling::Sniff)
    }
    /// Instantiates a new decoder for this encoding with BOM removal.
    ///
    /// If the input starts with bytes that are the BOM for this encoding,
    /// those bytes are removed. However, the decoder never morphs into a
    /// decoder for another encoding: A BOM for another encoding is treated as
    /// (potentially malformed) input to the decoding algorithm for this
    /// encoding.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn new_decoder_with_bom_removal(&'static self) -> Decoder {
        Decoder::new(self, self.new_variant_decoder(), BomHandling::Remove)
    }
    /// Instantiates a new decoder for this encoding with BOM handling disabled.
    ///
    /// If the input starts with bytes that look like a BOM, those bytes are
    /// not treated as a BOM. (Hence, the decoder never morphs into a decoder
    /// for another encoding.)
    ///
    /// _Note:_ If the caller has performed BOM sniffing on its own but has not
    /// removed the BOM, the caller should use `new_decoder_with_bom_removal()`
    /// instead of this method to cause the BOM to be removed.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn new_decoder_without_bom_handling(&'static self) -> Decoder {
        Decoder::new(self, self.new_variant_decoder(), BomHandling::Off)
    }
    /// Instantiates a new encoder for the [_output encoding_](Encoding::output_encoding)
    /// of this encoding.
    ///
    /// _Note:_ The output encoding of UTF-16BE, UTF-16LE, and replacement is UTF-8. There
    /// is no encoder for UTF-16BE, UTF-16LE, and replacement themselves.
    ///
    /// Available via the C wrapper.
    #[inline]
    pub fn new_encoder(&'static self) -> Encoder {
        let enc = self.output_encoding();
        enc.variant.new_encoder(enc)
    }
    /// Validates UTF-8.
    ///
    /// Returns the index of the first byte that makes the input malformed as
    /// UTF-8 or the length of the slice if the slice is entirely valid.
    ///
    /// This is currently faster than the corresponding standard library
    /// functionality. If this implementation gets upstreamed to the standard
    /// library, this method may be removed in the future.
    ///
    /// Available via the C wrapper.
    pub fn utf8_valid_up_to(bytes: &[u8]) -> usize {
        utf8_valid_up_to(bytes)
    }
    /// Validates ASCII.
    ///
    /// Returns the index of the first byte that makes the input malformed as
    /// ASCII or the length of the slice if the slice is entirely valid.
    ///
    /// Available via the C wrapper.
    pub fn ascii_valid_up_to(bytes: &[u8]) -> usize {
        ascii_valid_up_to(bytes)
    }
    /// Validates ISO-2022-JP ASCII-state data.
    ///
    /// Returns the index of the first byte that makes the input not
    /// representable in the ASCII state of ISO-2022-JP or the length of the
    /// slice if the slice is entirely representable in the ASCII state of
    /// ISO-2022-JP.
    ///
    /// Available via the C wrapper.
    pub fn iso_2022_jp_ascii_valid_up_to(bytes: &[u8]) -> usize {
        iso_2022_jp_ascii_valid_up_to(bytes)
    }
}
