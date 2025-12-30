// Generated macro for use_43 (pub_use)
macro_rules! Depcrate_smart_displayuse_43 {
() => {
// Module: crate::smart_display
// Provides: {"use_43"}
// Dependencies: {}
# [doc = " Implement [`Display`] for a type by using its implementation of [`SmartDisplay`]."] # [doc = ""] # [doc = " This attribute is applied to the `SmartDisplay` implementation."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use powerfmt::smart_display::{self, SmartDisplay, Metadata, FormatterOptions};"] # [doc = " # struct Foo;"] # [doc = " #[smart_display::delegate]"] # [doc = " impl SmartDisplay for Foo {"] # [doc = " #   type Metadata = ();"] # [doc = " #   fn metadata(&self, f: FormatterOptions) -> Metadata<Self> {"] # [doc = " #       todo!()"] # [doc = " #   }"] # [doc = "     // ..."] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "macros")] pub use powerfmt_macros :: smart_display_delegate as delegate ;
};
}
