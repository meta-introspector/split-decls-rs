macro_rules! UNICODE_PROPERTY_NAMES {
    () => {
        # [doc (hidden)] # [deprecated (note = "use `pest::unicode::unicode_property_names` instead")] pub static UNICODE_PROPERTY_NAMES : LazyLock < Vec < & str > > = LazyLock :: new (| | unicode_property_names () . collect :: < Vec < _ > > ()) ;
    };
}

UNICODE_PROPERTY_NAMES!()