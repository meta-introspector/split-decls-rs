macro_rules! FluentBundle {
    () => {
        pub type FluentBundle = IntoDynSyncSend < fluent_bundle :: bundle :: FluentBundle < FluentResource , IntlLangMemoizer > > ;
    };
}

FluentBundle!();