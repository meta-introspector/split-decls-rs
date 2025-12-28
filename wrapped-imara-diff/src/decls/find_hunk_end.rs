macro_rules! find_hunk_end {
    () => {
        pub fn find_hunk_end (changes : & [bool] , pos : u32) -> u32 { pos + changes [pos as usize ..] . iter () . take_while (| & & changed | changed) . count () as u32 }
    };
}

find_hunk_end!()