macro_rules! deps {
    () => {
        State!();
        Version!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl State { fn detect_required_version (& self) -> Version { self . entries . iter () . find_map (| e | e . flags . contains (entry :: Flags :: EXTENDED) . then_some (Version :: V3)) . unwrap_or (Version :: V2) } }
    };
}

impl_149!();