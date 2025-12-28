macro_rules! skip_while {
    () => {
        fn skip_while < P > (s : & str , mut predicate : P) -> & str where P : FnMut (char) -> bool , { let offset : usize = s . chars () . skip_while (| ch | predicate (* ch)) . map (| ch | ch . len_utf8 ()) . sum () ; & s [(s . len () - offset) ..] }
    };
}

skip_while!()