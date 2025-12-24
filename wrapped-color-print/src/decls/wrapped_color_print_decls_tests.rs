use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(test)]
mod tests {
    use std::fmt::Write as _;
    use super::*;
    #[cfg(feature = "terminfo")]
    pub mod color_print {
        pub use super::*;
    }
    #[test]
    fn format_no_arg() {
        assert_eq!(cformat!(), "");
        cprint!();
        cprintln!();
    }
    #[test]
    fn format_no_color() {
        assert_eq!(cformat!(""), "");
        assert_eq!(cformat!("Hi"), "Hi");
        assert_eq!(cformat!("Hi {}", 12), "Hi 12");
        assert_eq!(cformat!("Hi {n} {}", 12, n = 24), "Hi 24 12");
        let mut s = String::new();
        cwrite!(& mut s, "").unwrap();
        assert_eq!(s, "");
        let mut s = String::new();
        cwrite!(& mut s, "Hi").unwrap();
        assert_eq!(s, "Hi");
        let mut s = String::new();
        cwrite!(& mut s, "Hi {}", 12).unwrap();
        assert_eq!(s, "Hi 12");
        let mut s = String::new();
        cwrite!(& mut s, "Hi {n} {}", 12, n = 24).unwrap();
        assert_eq!(s, "Hi 24 12");
    }
    #[test]
    #[cfg(not(feature = "terminfo"))]
    #[rustfmt::skip]
    fn format_basic() {
        assert_eq!(cformat!("<red>Hi</red>"), "\u{1b}[31mHi\u{1b}[39m");
        assert_eq!(cformat!("<red>Hi</r>"), "\u{1b}[31mHi\u{1b}[39m");
        assert_eq!(cformat!("<red>Hi</>"), "\u{1b}[31mHi\u{1b}[39m");
        assert_eq!(cformat!("<bg:red>Hi</bg:red>"), "\u{1b}[41mHi\u{1b}[49m");
        assert_eq!(cformat!("<bg:red>Hi</R>"), "\u{1b}[41mHi\u{1b}[49m");
        assert_eq!(cformat!("<bg:red>Hi</>"), "\u{1b}[41mHi\u{1b}[49m");
        assert_eq!(cformat!("Hi <bold>word</bold> !"), "Hi \u{1b}[1mword\u{1b}[22m !");
        assert_eq!(cformat!("Hi <em>word</em> !"), "Hi \u{1b}[1mword\u{1b}[22m !");
        assert_eq!(cformat!("Hi <em>word</> !"), "Hi \u{1b}[1mword\u{1b}[22m !");
        assert_eq!(
            cformat!("
                <bold>bold</>
                <dim>dim</>
                <underline>underline</>
                <strike>strike</>
                <reverse>reverse</>
                <conceal>conceal</>
                <italics>italics</>
                <blink>blink</>
            "),
            "
                \u{1b}[1mbold\u{1b}[22m
                \u{1b}[2mdim\u{1b}[22m
                \u{1b}[4munderline\u{1b}[24m
                \u{1b}[9mstrike\u{1b}[29m
                \u{1b}[7mreverse\u{1b}[27m
                \u{1b}[8mconceal\u{1b}[28m
                \u{1b}[3mitalics\u{1b}[23m
                \u{1b}[5mblink\u{1b}[25m
            "
        );
        let mut s = String::new();
        cwrite!(& mut s, "Hi <r>{v}</> {}", 12, v = "Hi").unwrap();
        assert_eq!(s, "Hi \u{1b}[31mHi\u{1b}[39m 12");
        let mut s = String::new();
        cwriteln!(& mut s, "Hi <r>{v} {}", 12, v = "Hi").unwrap();
        assert_eq!(s, "Hi \u{1b}[31mHi 12\u{1b}[39m\n");
    }
    #[test]
    #[ignore]
    #[cfg(not(feature = "terminfo"))]
    fn bold_and_dim_should_be_optimized() {
        assert_eq!(
            cformat!("<bold>BOLD</><dim>DIM</>"), "\u{1b}[1mBOLD\u{1b}[2mDIM\u{1b}[22m"
        );
    }
    #[test]
    #[cfg(not(feature = "terminfo"))]
    fn format_multiple() {
        assert_eq!(
            cformat!("Hi <bold>word</bold> <red>red</red> !"),
            "Hi \u{1b}[1mword\u{1b}[22m \u{1b}[31mred\u{1b}[39m !"
        );
    }
    #[test]
    #[cfg(not(feature = "terminfo"))]
    fn format_optimization() {
        assert_eq!(
            cformat!("<red>RED<blue>BLUE</>RED</>"),
            "\u{1b}[31mRED\u{1b}[34mBLUE\u{1b}[31mRED\u{1b}[39m"
        );
        assert_eq!(
            cformat!("<red><blue>BLUE</>RED</>"), "\u{1b}[34mBLUE\u{1b}[31mRED\u{1b}[39m"
        );
        assert_eq!(cformat!("<red></>Text"), "Text");
    }
    #[test]
    #[cfg(not(feature = "terminfo"))]
    #[rustfmt::skip]
    fn format_auto_close_tag() {
        assert_eq!(
            cformat!("<red>RED<blue>BLUE"), "\u{1b}[31mRED\u{1b}[34mBLUE\u{1b}[39m"
        );
        assert!(
            cformat!("<red>RED<em>BOLD") ==
            "\u{1b}[31mRED\u{1b}[1mBOLD\u{1b}[22m\u{1b}[39m" ||
            cformat!("<red>RED<em>BOLD") ==
            "\u{1b}[31mRED\u{1b}[1mBOLD\u{1b}[39m\u{1b}[22m"
        );
    }
    #[test]
    #[cfg(feature = "terminfo")]
    fn terminfo_format_basic() {
        assert_eq!(cformat!("<red>Hi</red>"), format!("{}Hi{}", * RED, * CLEAR));
        assert_eq!(
            cformat!("Hi <bold>word</bold> !"), format!("Hi {}word{} !", * BOLD, * CLEAR)
        );
        let mut s = String::new();
        cwrite!(& mut s, "<r>Hi</> {}", 12).unwrap();
        assert_eq!(s, format!("{}Hi{} 12", * RED, * CLEAR));
    }
    #[test]
    #[cfg(feature = "terminfo")]
    fn terminfo_format_multiple() {
        assert_eq!(
            cformat!("Hi <bold>word</bold> <red>red</red> !"),
            format!("Hi {}word{} {}red{} !", * BOLD, * CLEAR, * RED, * CLEAR)
        );
    }
    #[test]
    #[cfg(feature = "terminfo")]
    fn terminfo_format_auto_close_tag() {
        assert_eq!(
            cformat!("<red>RED<blue>BLUE"), format!("{}RED{}BLUE{}", * RED, * BLUE, *
            CLEAR)
        );
        assert_eq!(
            cformat!("<red>RED<em>BOLD"), format!("{}RED{}BOLD{}", * RED, * BOLD, *
            CLEAR)
        );
    }
    #[test]
    fn untagged() {
        assert_eq!(untagged!(""), "");
        assert_eq!(untagged!("hi"), "hi");
        assert_eq!(untagged!("<red>hi"), "hi");
        assert_eq!(untagged!("<red>hi</>"), "hi");
        assert_eq!(untagged!("<red>hi <em,blue>all"), "hi all");
        assert_eq!(untagged!("<red>hi <em>all</></>"), "hi all");
    }
}
