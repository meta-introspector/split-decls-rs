macro_rules! path_segments {
    () => {
        fn path_segments (path : & BStr) -> Option < impl Iterator < Item = & [u8] > > { if path . starts_with (b"/") { Some (path [1 ..] . split (| c | * c == b'/')) } else { None } }
    };
}

path_segments!()