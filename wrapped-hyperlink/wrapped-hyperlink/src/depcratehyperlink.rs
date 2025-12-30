// Generated macro for Hyperlink (struct)
macro_rules! DepcrateHyperlink {
() => {
// Module: crate
// Provides: {"Hyperlink"}
// Dependencies: {}
# [doc = " A hyperlink widget that renders a hyperlink in the terminal using [OSC 8]."] # [doc = ""] # [doc = " [OSC 8]: https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda"] struct Hyperlink < 'content > { text : Text < 'content > , url : String , }
};
}
