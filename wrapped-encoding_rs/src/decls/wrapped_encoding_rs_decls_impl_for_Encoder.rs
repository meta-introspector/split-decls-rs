use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl Encoder {
    fn new(enc: &'static Encoding, encoder: VariantEncoder) -> Encoder {
        Encoder {
            encoding: enc,
            variant: encoder,
        }
    }
    /// The `Encoding` this `Encoder` is for.
    #[inline]
    pub fn encoding(&self) -> &'static Encoding {
        self.encoding
    }
    /// Returns `true` if this is an ISO-2022-JP encoder that's not in the
    /// ASCII state and `false` otherwise.
    #[inline]
    pub fn has_pending_state(&self) -> bool {
        self.variant.has_pending_state()
    }
    /// Query the worst-case output size when encoding from UTF-8 with
    /// replacement.
    ///
    /// Returns the size of the output buffer in bytes that will not overflow
    /// given the current state of the encoder and `byte_length` number of
    /// additional input code units if there are no unmappable characters in
    /// the input or `None` if `usize` would overflow.
    ///
    /// Available via the C wrapper.
    pub fn max_buffer_length_from_utf8_if_no_unmappables(
        &self,
        byte_length: usize,
    ) -> Option<usize> {
        checked_add(
            if self.encoding().can_encode_everything() { 0 } else { NCR_EXTRA },
            self.max_buffer_length_from_utf8_without_replacement(byte_length),
        )
    }
    /// Query the worst-case output size when encoding from UTF-8 without
    /// replacement.
    ///
    /// Returns the size of the output buffer in bytes that will not overflow
    /// given the current state of the encoder and `byte_length` number of
    /// additional input code units or `None` if `usize` would overflow.
    ///
    /// Available via the C wrapper.
    pub fn max_buffer_length_from_utf8_without_replacement(
        &self,
        byte_length: usize,
    ) -> Option<usize> {
        self.variant.max_buffer_length_from_utf8_without_replacement(byte_length)
    }
    /// Incrementally encode into byte stream from UTF-8 with unmappable
    /// characters replaced with HTML (decimal) numeric character references.
    ///
    /// See the documentation of the struct for documentation for `encode_*`
    /// methods collectively.
    ///
    /// Available via the C wrapper.
    pub fn encode_from_utf8(
        &mut self,
        src: &str,
        dst: &mut [u8],
        last: bool,
    ) -> (CoderResult, usize, usize, bool) {
        let dst_len = dst.len();
        let effective_dst_len = if self.encoding().can_encode_everything() {
            dst_len
        } else {
            if dst_len < NCR_EXTRA {
                if src.is_empty() && !(last && self.has_pending_state()) {
                    return (CoderResult::InputEmpty, 0, 0, false);
                }
                return (CoderResult::OutputFull, 0, 0, false);
            }
            dst_len - NCR_EXTRA
        };
        let mut had_unmappables = false;
        let mut total_read = 0usize;
        let mut total_written = 0usize;
        loop {
            let (result, read, written) = self
                .encode_from_utf8_without_replacement(
                    &src[total_read..],
                    &mut dst[total_written..effective_dst_len],
                    last,
                );
            total_read += read;
            total_written += written;
            match result {
                EncoderResult::InputEmpty => {
                    return (
                        CoderResult::InputEmpty,
                        total_read,
                        total_written,
                        had_unmappables,
                    );
                }
                EncoderResult::OutputFull => {
                    return (
                        CoderResult::OutputFull,
                        total_read,
                        total_written,
                        had_unmappables,
                    );
                }
                EncoderResult::Unmappable(unmappable) => {
                    had_unmappables = true;
                    debug_assert!(dst.len() - total_written >= NCR_EXTRA);
                    debug_assert_ne!(self.encoding(), UTF_16BE);
                    debug_assert_ne!(self.encoding(), UTF_16LE);
                    total_written += write_ncr(unmappable, &mut dst[total_written..]);
                    if total_written >= effective_dst_len {
                        if total_read == src.len() && !(last && self.has_pending_state())
                        {
                            return (
                                CoderResult::InputEmpty,
                                total_read,
                                total_written,
                                had_unmappables,
                            );
                        }
                        return (
                            CoderResult::OutputFull,
                            total_read,
                            total_written,
                            had_unmappables,
                        );
                    }
                }
            }
        }
    }
    /// Incrementally encode into byte stream from UTF-8 with unmappable
    /// characters replaced with HTML (decimal) numeric character references.
    ///
    /// See the documentation of the struct for documentation for `encode_*`
    /// methods collectively.
    ///
    /// Available to Rust only and only with the `alloc` feature enabled (enabled
    /// by default).
    #[cfg(feature = "alloc")]
    pub fn encode_from_utf8_to_vec(
        &mut self,
        src: &str,
        dst: &mut Vec<u8>,
        last: bool,
    ) -> (CoderResult, usize, bool) {
        unsafe {
            let old_len = dst.len();
            let capacity = dst.capacity();
            dst.set_len(capacity);
            let (result, read, written, replaced) = self
                .encode_from_utf8(src, &mut dst[old_len..], last);
            dst.set_len(old_len + written);
            (result, read, replaced)
        }
    }
    /// Incrementally encode into byte stream from UTF-8 _without replacement_.
    ///
    /// See the documentation of the struct for documentation for `encode_*`
    /// methods collectively.
    ///
    /// Available via the C wrapper.
    pub fn encode_from_utf8_without_replacement(
        &mut self,
        src: &str,
        dst: &mut [u8],
        last: bool,
    ) -> (EncoderResult, usize, usize) {
        self.variant.encode_from_utf8_raw(src, dst, last)
    }
    /// Incrementally encode into byte stream from UTF-8 _without replacement_.
    ///
    /// See the documentation of the struct for documentation for `encode_*`
    /// methods collectively.
    ///
    /// Available to Rust only and only with the `alloc` feature enabled (enabled
    /// by default).
    #[cfg(feature = "alloc")]
    pub fn encode_from_utf8_to_vec_without_replacement(
        &mut self,
        src: &str,
        dst: &mut Vec<u8>,
        last: bool,
    ) -> (EncoderResult, usize) {
        unsafe {
            let old_len = dst.len();
            let capacity = dst.capacity();
            dst.set_len(capacity);
            let (result, read, written) = self
                .encode_from_utf8_without_replacement(src, &mut dst[old_len..], last);
            dst.set_len(old_len + written);
            (result, read)
        }
    }
    /// Query the worst-case output size when encoding from UTF-16 with
    /// replacement.
    ///
    /// Returns the size of the output buffer in bytes that will not overflow
    /// given the current state of the encoder and `u16_length` number of
    /// additional input code units if there are no unmappable characters in
    /// the input or `None` if `usize` would overflow.
    ///
    /// Available via the C wrapper.
    pub fn max_buffer_length_from_utf16_if_no_unmappables(
        &self,
        u16_length: usize,
    ) -> Option<usize> {
        checked_add(
            if self.encoding().can_encode_everything() { 0 } else { NCR_EXTRA },
            self.max_buffer_length_from_utf16_without_replacement(u16_length),
        )
    }
    /// Query the worst-case output size when encoding from UTF-16 without
    /// replacement.
    ///
    /// Returns the size of the output buffer in bytes that will not overflow
    /// given the current state of the encoder and `u16_length` number of
    /// additional input code units or `None` if `usize` would overflow.
    ///
    /// Available via the C wrapper.
    pub fn max_buffer_length_from_utf16_without_replacement(
        &self,
        u16_length: usize,
    ) -> Option<usize> {
        self.variant.max_buffer_length_from_utf16_without_replacement(u16_length)
    }
    /// Incrementally encode into byte stream from UTF-16 with unmappable
    /// characters replaced with HTML (decimal) numeric character references.
    ///
    /// See the documentation of the struct for documentation for `encode_*`
    /// methods collectively.
    ///
    /// Available via the C wrapper.
    pub fn encode_from_utf16(
        &mut self,
        src: &[u16],
        dst: &mut [u8],
        last: bool,
    ) -> (CoderResult, usize, usize, bool) {
        let dst_len = dst.len();
        let effective_dst_len = if self.encoding().can_encode_everything() {
            dst_len
        } else {
            if dst_len < NCR_EXTRA {
                if src.is_empty() && !(last && self.has_pending_state()) {
                    return (CoderResult::InputEmpty, 0, 0, false);
                }
                return (CoderResult::OutputFull, 0, 0, false);
            }
            dst_len - NCR_EXTRA
        };
        let mut had_unmappables = false;
        let mut total_read = 0usize;
        let mut total_written = 0usize;
        loop {
            let (result, read, written) = self
                .encode_from_utf16_without_replacement(
                    &src[total_read..],
                    &mut dst[total_written..effective_dst_len],
                    last,
                );
            total_read += read;
            total_written += written;
            match result {
                EncoderResult::InputEmpty => {
                    return (
                        CoderResult::InputEmpty,
                        total_read,
                        total_written,
                        had_unmappables,
                    );
                }
                EncoderResult::OutputFull => {
                    return (
                        CoderResult::OutputFull,
                        total_read,
                        total_written,
                        had_unmappables,
                    );
                }
                EncoderResult::Unmappable(unmappable) => {
                    had_unmappables = true;
                    debug_assert!(dst.len() - total_written >= NCR_EXTRA);
                    debug_assert_ne!(self.encoding(), UTF_16BE);
                    debug_assert_ne!(self.encoding(), UTF_16LE);
                    total_written += write_ncr(unmappable, &mut dst[total_written..]);
                    if total_written >= effective_dst_len {
                        if total_read == src.len() && !(last && self.has_pending_state())
                        {
                            return (
                                CoderResult::InputEmpty,
                                total_read,
                                total_written,
                                had_unmappables,
                            );
                        }
                        return (
                            CoderResult::OutputFull,
                            total_read,
                            total_written,
                            had_unmappables,
                        );
                    }
                }
            }
        }
    }
    /// Incrementally encode into byte stream from UTF-16 _without replacement_.
    ///
    /// See the documentation of the struct for documentation for `encode_*`
    /// methods collectively.
    ///
    /// Available via the C wrapper.
    pub fn encode_from_utf16_without_replacement(
        &mut self,
        src: &[u16],
        dst: &mut [u8],
        last: bool,
    ) -> (EncoderResult, usize, usize) {
        self.variant.encode_from_utf16_raw(src, dst, last)
    }
}
