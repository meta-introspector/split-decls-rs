// Generated macro for FormatterOptions (struct)
macro_rules! Depcrate_smart_displayFormatterOptions {
() => {
// Module: crate::smart_display
// Provides: {"FormatterOptions"}
// Dependencies: {}
# [doc = " Configuration for formatting."] # [doc = ""] # [doc = " This struct is obtained from a [`Formatter`]. It provides the same functionality as that of a"] # [doc = " reference to a `Formatter`. However, it is not possible to construct a `Formatter`, which is"] # [doc = " necessary for some use cases of [`SmartDisplay`]. `FormatterOptions` implements [`Default`] and"] # [doc = " has builder methods to alleviate this."] # [derive (Clone , Copy)] pub struct FormatterOptions { flags : u8 , fill : char , align : Option < Alignment > , width : MaybeUninit < usize > , precision : MaybeUninit < usize > , }
};
}
