macro_rules! deps {
    () => {
        OwnedPcwstr!();
    };
}

macro_rules! multi_pcwstr {
    () => {
        deps!();
        pub fn multi_pcwstr < T : AsRef < str > > (value : & [T]) -> OwnedPcwstr { OwnedPcwstr (value . iter () . flat_map (| value | value . as_ref () . encode_utf16 () . chain (core :: iter :: once (0))) . chain (core :: iter :: once (0)) . collect () ,) }
    };
}

multi_pcwstr!()