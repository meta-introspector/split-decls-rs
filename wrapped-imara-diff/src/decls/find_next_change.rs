macro_rules! find_next_change {
    () => {
        pub fn find_next_change (changes : & [bool] , pos : u32) -> Option < u32 > { changes [pos as usize ..] . iter () . position (| & changed | changed) . map (| off | off as u32) }
    };
}

find_next_change!()