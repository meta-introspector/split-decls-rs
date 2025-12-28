macro_rules! deps {
    () => {
        Hunk!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl Hunk { pub (crate) fn next_hunk (& mut self , removed : & [bool] , added : & [bool]) -> bool { let Some (off) = find_next_change (added , self . after . end) else { return false ; } ; let mut off_before = 0 ; loop { debug_assert ! (removed . len () as u32 != self . before . end || off == 0 , "broken hunk alignment {self:?} ") ; let unchanged_tokens = find_next_change (removed , self . before . end) . unwrap_or (removed . len () as u32 - self . before . end) ; if off_before + unchanged_tokens > off { self . before . start = self . before . end + (off - off_before) ; self . before . end = self . before . start ; break ; } off_before += unchanged_tokens ; self . before . start = self . before . end + unchanged_tokens ; self . before . end = find_hunk_end (removed , self . before . end + unchanged_tokens) ; if off_before == off { break ; } } self . after . start = self . after . end + off ; self . after . end = find_hunk_end (added , self . after . start) ; true } }
    };
}

impl_91!()