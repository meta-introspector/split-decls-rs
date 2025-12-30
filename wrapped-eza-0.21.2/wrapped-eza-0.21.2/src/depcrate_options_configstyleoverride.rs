// Generated macro for StyleOverride (struct)
macro_rules! Depcrate_options_configStyleOverride {
() => {
// Module: crate::options::config
// Provides: {"StyleOverride"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone , Copy , Serialize , Deserialize , Default)] pub struct StyleOverride { # [doc = " The style's foreground color, if it has one."] # [serde (alias = "fg" , deserialize_with = "deserialize_color" , default)] pub foreground : Option < Color > , # [doc = " The style's background color, if it has one."] # [serde (alias = "bg" , deserialize_with = "deserialize_color" , default)] pub background : Option < Color > , # [doc = " Whether this style is bold."] # [serde (alias = "bold")] pub is_bold : Option < bool > , # [doc = " Whether this style is dimmed."] # [serde (alias = "dimmed")] pub is_dimmed : Option < bool > , # [doc = " Whether this style is italic."] # [serde (alias = "italic")] pub is_italic : Option < bool > , # [doc = " Whether this style is underlined."] # [serde (alias = "underline")] pub is_underline : Option < bool > , # [doc = " Whether this style is blinking."] # [serde (alias = "blink")] pub is_blink : Option < bool > , # [doc = " Whether this style has reverse colors."] # [serde (alias = "reverse")] pub is_reverse : Option < bool > , # [doc = " Whether this style is hidden."] # [serde (alias = "hidden")] pub is_hidden : Option < bool > , # [doc = " Whether this style is struckthrough."] # [serde (alias = "strikethrough")] pub is_strikethrough : Option < bool > , # [doc = " Wether this style is always displayed starting with a reset code to clear any remaining style artifacts"] # [serde (alias = "prefix_reset")] pub prefix_with_reset : Option < bool > , }
};
}
