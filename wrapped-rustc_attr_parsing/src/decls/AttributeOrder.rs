macro_rules! AttributeOrder {
    () => {
        pub (crate) enum AttributeOrder { # [doc = " Duplicates after the innermost instance of the attribute will be an error/warning."] # [doc = " Only keep the lowest attribute."] # [doc = ""] # [doc = " Attributes are processed from bottom to top, so this raises a warning/error on all the attributes"] # [doc = " further above the lowest one:"] # [doc = " ```"] # [doc = " #[stable(since=\"1.0\")] //~ WARNING duplicated attribute"] # [doc = " #[stable(since=\"2.0\")]"] # [doc = " ```"] KeepInnermost , # [doc = " Duplicates before the outermost instance of the attribute will be an error/warning."] # [doc = " Only keep the highest attribute."] # [doc = ""] # [doc = " Attributes are processed from bottom to top, so this raises a warning/error on all the attributes"] # [doc = " below the highest one:"] # [doc = " ```"] # [doc = " #[path=\"foo.rs\"]"] # [doc = " #[path=\"bar.rs\"] //~ WARNING duplicated attribute"] # [doc = " ```"] KeepOutermost , }
    };
}

AttributeOrder!();