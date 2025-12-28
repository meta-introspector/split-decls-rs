macro_rules! deps {
    () => {
        HighlightMap!();
    };
}

macro_rules! merge_map {
    () => {
        deps!();
        fn merge_map (res : & mut HighlightMap , new : Option < HighlightMap >) { let Some (new) = new else { return ; } ; new . into_iter () . for_each (| (file_id , ranges) | { res . entry (file_id) . or_default () . extend (ranges) ; }) ; }
    };
}

merge_map!()