macro_rules! macro_32 {
    () => {
        rustc_fluent_macro :: fluent_messages ! { "../messages.ftl" }
    };
}

macro_32!()