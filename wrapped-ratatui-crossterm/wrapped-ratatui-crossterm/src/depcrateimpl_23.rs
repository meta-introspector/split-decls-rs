// Generated macro for impl_23 (impl)
macro_rules! Depcrateimpl_23 {
() => {
// Module: crate
// Provides: {"impl_23"}
// Dependencies: {}
impl FromCrossterm < CrosstermAttributes > for Modifier { fn from_crossterm (value : CrosstermAttributes) -> Self { let mut res = Self :: empty () ; if value . has (CrosstermAttribute :: Bold) { res |= Self :: BOLD ; } if value . has (CrosstermAttribute :: Dim) { res |= Self :: DIM ; } if value . has (CrosstermAttribute :: Italic) { res |= Self :: ITALIC ; } if value . has (CrosstermAttribute :: Underlined) || value . has (CrosstermAttribute :: DoubleUnderlined) || value . has (CrosstermAttribute :: Undercurled) || value . has (CrosstermAttribute :: Underdotted) || value . has (CrosstermAttribute :: Underdashed) { res |= Self :: UNDERLINED ; } if value . has (CrosstermAttribute :: SlowBlink) { res |= Self :: SLOW_BLINK ; } if value . has (CrosstermAttribute :: RapidBlink) { res |= Self :: RAPID_BLINK ; } if value . has (CrosstermAttribute :: Reverse) { res |= Self :: REVERSED ; } if value . has (CrosstermAttribute :: Hidden) { res |= Self :: HIDDEN ; } if value . has (CrosstermAttribute :: CrossedOut) { res |= Self :: CROSSED_OUT ; } res } }
};
}
