macro_rules! deps {
    () => {
        Key!();
        Section!();
        Gitoxide!();
    };
}

macro_rules! impl_657 {
    () => {
        deps!();
        impl Section for Gitoxide { fn name (& self) -> & str { "gitoxide" } fn keys (& self) -> & [& dyn Key] { & [& Self :: USER_AGENT , & Self :: TRACE_PACKET , & Self :: PARSE_PRECIOUS] } fn sub_sections (& self) -> & [& dyn Section] { & [& Self :: ALLOW , & Self :: AUTHOR , & Self :: CORE , & Self :: COMMIT , & Self :: COMMITTER , & Self :: CREDENTIALS , & Self :: HTTP , & Self :: HTTPS , & Self :: OBJECTS , & Self :: SSH , & Self :: USER , & Self :: PATHSPEC ,] } }
    };
}

impl_657!()