macro_rules! take_while {
    () => {
        fn take_while < P > (s : & str , mut predicate : P) -> & str where P : FnMut (char) -> bool , { let offset = s . chars () . take_while (| ch | predicate (* ch)) . map (| ch | ch . len_utf8 ()) . sum () ; & s [.. offset] }
    };
}

take_while!();