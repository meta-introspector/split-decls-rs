macro_rules! to_components_bstring_ref {
    () => {
        pub fn to_components_bstring_ref (rela_path : & BString) -> impl Iterator < Item = & BStr > { rela_path . split (| b | * b == b'/') . map (Into :: into) }
    };
}

to_components_bstring_ref!();