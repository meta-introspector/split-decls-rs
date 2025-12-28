macro_rules! write_conflict_marker {
    () => {
        pub fn write_conflict_marker (out : & mut Vec < u8 > , marker : u8 , label : Option < & BStr > , marker_size : u8 , nl : & BStr) { assure_ends_with_nl (out , nl) ; out . extend (std :: iter :: repeat_n (marker , marker_size as usize)) ; if let Some (label) = label { out . push (b' ') ; out . extend_from_slice (label) ; } out . push_str (nl) ; }
    };
}

write_conflict_marker!()