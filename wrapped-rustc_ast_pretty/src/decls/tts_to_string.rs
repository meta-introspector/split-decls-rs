macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! tts_to_string {
    () => {
        deps!();
        pub fn tts_to_string (tokens : & TokenStream) -> String { State :: new () . tts_to_string (tokens) }
    };
}

tts_to_string!();