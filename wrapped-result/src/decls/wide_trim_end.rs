macro_rules! wide_trim_end {
    () => {
        pub fn wide_trim_end (mut wide : & [u16]) -> & [u16] { while let Some (last) = wide . last () { match last { 32 | 9 ..= 13 => wide = & wide [.. wide . len () - 1] , _ => break , } } wide }
    };
}

wide_trim_end!()