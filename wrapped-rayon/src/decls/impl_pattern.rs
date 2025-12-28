macro_rules! deps {
    () => {
        Folder!();
    };
}

macro_rules! impl_pattern {
    () => {
        deps!();
        macro_rules ! impl_pattern { (&$ self : ident => $ pattern : expr) => { private_impl ! { } # [inline] fn find_in (&$ self , chars : & str) -> Option < usize > { chars . find ($ pattern) } # [inline] fn rfind_in (&$ self , chars : & str) -> Option < usize > { chars . rfind ($ pattern) } # [inline] fn is_suffix_of (&$ self , chars : & str) -> bool { chars . ends_with ($ pattern) } fn fold_splits <'ch , F > (&$ self , chars : &'ch str , folder : F , skip_last : bool) -> F where F : Folder <&'ch str >, { let mut split = chars . split ($ pattern) ; if skip_last { split . next_back () ; } folder . consume_iter (split) } fn fold_inclusive_splits <'ch , F > (&$ self , chars : &'ch str , folder : F) -> F where F : Folder <&'ch str >, { folder . consume_iter (chars . split_inclusive ($ pattern)) } fn fold_matches <'ch , F > (&$ self , chars : &'ch str , folder : F) -> F where F : Folder <&'ch str >, { folder . consume_iter (chars . matches ($ pattern)) } fn fold_match_indices <'ch , F > (&$ self , chars : &'ch str , folder : F , base : usize) -> F where F : Folder < (usize , &'ch str) >, { folder . consume_iter (chars . match_indices ($ pattern) . map (offset (base))) } } }
    };
}

impl_pattern!();