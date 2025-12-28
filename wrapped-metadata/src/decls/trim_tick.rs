macro_rules! trim_tick {
    () => {
        fn trim_tick (name : & str) -> & str { if name . as_bytes () . iter () . rev () . nth (1) == Some (& b'`') { & name [.. name . len () - 2] } else { name } }
    };
}

trim_tick!()