macro_rules! block_count_sans_ansi_codes {
    () => {
        fn block_count_sans_ansi_codes (strings : & [ANSIString < '_ >]) -> u16 { strings . iter () . map (| s | s . width () as u16) . sum () }
    };
}

block_count_sans_ansi_codes!();