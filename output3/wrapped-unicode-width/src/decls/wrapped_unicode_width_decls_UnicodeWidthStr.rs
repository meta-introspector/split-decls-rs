use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Methods for determining displayed width of Unicode strings.
pub trait UnicodeWidthStr: private::Sealed {
    /// Returns the string's displayed width in columns.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 1 column wide. This is consistent with the recommendations for
    /// non-CJK contexts, or when the context cannot be reliably determined.
    fn width(&self) -> usize;
    /// Returns the string's displayed width in columns.
    ///
    /// This function treats characters in the Ambiguous category according
    /// to [Unicode Standard Annex #11](http://www.unicode.org/reports/tr11/)
    /// as 2 column wide. This is consistent with the recommendations for
    /// CJK contexts.
    #[cfg(feature = "cjk")]
    fn width_cjk(&self) -> usize;
}
