macro_rules! to_components {
    () => {
        pub fn to_components (rela_path : & BStr) -> impl Iterator < Item = & BStr > { rela_path . split (| b | * b == b'/') . map (Into :: into) }
    };
}

to_components!();