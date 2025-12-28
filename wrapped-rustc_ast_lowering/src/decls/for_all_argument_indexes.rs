macro_rules! for_all_argument_indexes {
    () => {
        fn for_all_argument_indexes (template : & mut [FormatArgsPiece] , mut f : impl FnMut (& mut usize)) { for piece in template { let FormatArgsPiece :: Placeholder (placeholder) = piece else { continue } ; if let Ok (index) = & mut placeholder . argument . index { f (index) ; } if let Some (FormatCount :: Argument (FormatArgPosition { index : Ok (index) , .. })) = & mut placeholder . format_options . width { f (index) ; } if let Some (FormatCount :: Argument (FormatArgPosition { index : Ok (index) , .. })) = & mut placeholder . format_options . precision { f (index) ; } } }
    };
}

for_all_argument_indexes!()