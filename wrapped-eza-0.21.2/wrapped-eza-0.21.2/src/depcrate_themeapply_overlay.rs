// Generated macro for apply_overlay (function)
macro_rules! Depcrate_themeapply_overlay {
() => {
// Module: crate::theme
// Provides: {"apply_overlay"}
// Dependencies: {}
# [doc = " Some of the styles are **overlays**: although they have the same attribute"] # [doc = " set as regular styles (foreground and background colours, bold, underline,"] # [doc = " etc), they’re intended to be used to *amend* existing styles."] # [doc = ""] # [doc = " For example, the target path of a broken symlink is displayed in a red,"] # [doc = " underlined style by default. Paths can contain control characters, so"] # [doc = " these control characters need to be underlined too, otherwise it looks"] # [doc = " weird. So instead of having four separate configurable styles for “link"] # [doc = " path”, “broken link path”, “control character” and “broken control"] # [doc = " character”, there are styles for “link path”, “control character”, and"] # [doc = " “broken link overlay”, the latter of which is just set to override the"] # [doc = " underline attribute on the other two."] # [rustfmt :: skip] fn apply_overlay (mut base : Style , overlay : Style) -> Style { if let Some (fg) = overlay . foreground { base . foreground = Some (fg) ; } if let Some (bg) = overlay . background { base . background = Some (bg) ; } if overlay . is_bold { base . is_bold = true ; } if overlay . is_dimmed { base . is_dimmed = true ; } if overlay . is_italic { base . is_italic = true ; } if overlay . is_underline { base . is_underline = true ; } if overlay . is_blink { base . is_blink = true ; } if overlay . is_reverse { base . is_reverse = true ; } if overlay . is_hidden { base . is_hidden = true ; } if overlay . is_strikethrough { base . is_strikethrough = true ; } base }
};
}
