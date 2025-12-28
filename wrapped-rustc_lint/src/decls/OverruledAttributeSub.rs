macro_rules! OverruledAttributeSub {
    () => {
        pub (crate) enum OverruledAttributeSub { DefaultSource { id : String } , NodeSource { span : Span , reason : Option < Symbol > } , CommandLineSource , }
    };
}

OverruledAttributeSub!();