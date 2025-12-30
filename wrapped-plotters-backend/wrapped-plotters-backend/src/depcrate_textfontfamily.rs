// Generated macro for FontFamily (enum)
macro_rules! Depcrate_textFontFamily {
() => {
// Module: crate::text
// Provides: {"FontFamily"}
// Dependencies: {}
# [doc = " Describes font family."] # [doc = " This can be either a specific font family name, such as \"arial\","] # [doc = " or a general font family class, such as \"serif\" and \"sans-serif\""] # [derive (Clone , Copy)] pub enum FontFamily < 'a > { # [doc = " The system default serif font family"] Serif , # [doc = " The system default sans-serif font family"] SansSerif , # [doc = " The system default monospace font"] Monospace , # [doc = " A specific font family name"] Name (& 'a str) , }
};
}
