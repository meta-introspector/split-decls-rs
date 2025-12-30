// Generated macro for impl_474 (impl)
macro_rules! Depcrate_options_configimpl_474 {
() => {
// Module: crate::options::config
// Provides: {"impl_474"}
// Dependencies: {}
impl FromOverride < StyleOverride > for Style { fn from (value : StyleOverride , default : Self) -> Self { let mut style = default ; if value . foreground . is_some () { style . foreground = value . foreground ; } if value . background . is_some () { style . background = value . background ; } if let Some (bold) = value . is_bold { style . is_bold = bold ; } if let Some (dimmed) = value . is_dimmed { style . is_dimmed = dimmed ; } if let Some (italic) = value . is_italic { style . is_italic = italic ; } if let Some (underline) = value . is_underline { style . is_underline = underline ; } if let Some (blink) = value . is_blink { style . is_blink = blink ; } if let Some (reverse) = value . is_reverse { style . is_reverse = reverse ; } if let Some (hidden) = value . is_hidden { style . is_hidden = hidden ; } if let Some (strikethrough) = value . is_strikethrough { style . is_strikethrough = strikethrough ; } if let Some (reset) = value . prefix_with_reset { style . prefix_with_reset = reset ; } style } }
};
}
