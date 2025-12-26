use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl SmolStr {
    /// Constructs an inline variant of `SmolStr`.
    ///
    /// This never allocates.
    ///
    /// # Panics
    ///
    /// Panics if `text.len() > 23`.
    #[inline]
    pub const fn new_inline(text: &str) -> SmolStr {
        assert!(text.len() <= INLINE_CAP);
        let text = text.as_bytes();
        let mut buf = [0; INLINE_CAP];
        let mut i = 0;
        while i < text.len() {
            buf[i] = text[i];
            i += 1;
        }
        SmolStr(Repr::Inline {
            len: unsafe { InlineSize::transmute_from_u8(text.len() as u8) },
            buf,
        })
    }
    /// Constructs a `SmolStr` from a statically allocated string.
    ///
    /// This never allocates.
    #[inline(always)]
    pub const fn new_static(text: &'static str) -> SmolStr {
        SmolStr(Repr::Static(text))
    }
    /// Constructs a `SmolStr` from a `str`, heap-allocating if necessary.
    #[inline(always)]
    pub fn new(text: impl AsRef<str>) -> SmolStr {
        SmolStr(Repr::new(text.as_ref()))
    }
    /// Returns a `&str` slice of this `SmolStr`.
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    /// Returns the length of `self` in bytes.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Returns `true` if `self` has a length of zero bytes.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Returns `true` if `self` is heap-allocated.
    #[inline(always)]
    pub const fn is_heap_allocated(&self) -> bool {
        matches!(self.0, Repr::Heap(..))
    }
}
