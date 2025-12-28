macro_rules! deps {
    () => {
        Outcome!();
        Match!();
        TrackedAssignment!();
        MatchLocation!();
        MatchKind!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        # [doc = " Mutation"] impl Outcome { # [doc = " Fill all `attrs` and resolve them recursively if they are macros. Return `true` if there is no attribute left to be resolved and"] # [doc = " we are totally done."] # [doc = " `pattern` is what matched a patch and is passed for contextual information,"] # [doc = " providing `sequence_number` and `source` as well."] pub (crate) fn fill_attributes < 'a > (& mut self , attrs : impl Iterator < Item = & 'a TrackedAssignment > , pattern : & gix_glob :: Pattern , source : Option < & std :: path :: PathBuf > , sequence_number : usize ,) -> bool { self . attrs_stack . extend (attrs . filter (| attr | self . matches_by_id [attr . id . 0] . r#match . is_none ()) . map (| attr | (attr . id , attr . inner . clone () , None)) ,) ; while let Some ((id , assignment , parent_order)) = self . attrs_stack . pop () { let slot = & mut self . matches_by_id [id . 0] ; if slot . r#match . is_some () { continue ; } let is_macro = ! slot . macro_attributes . is_empty () ; slot . r#match = Some (Match { pattern : self . patterns . insert (pattern) , assignment : self . assignments . insert_owned (assignment) , kind : if is_macro { MatchKind :: Macro { parent_macro_id : parent_order , } } else { MatchKind :: Attribute { macro_id : parent_order } } , location : MatchLocation { source : source . map (| path | self . source_paths . insert (path)) , sequence_number , } , }) ; if self . reduce_and_check_if_done (id) { return true ; } if is_macro { let slot = & self . matches_by_id [id . 0] ; self . attrs_stack . extend (slot . macro_attributes . iter () . filter (| attr | self . matches_by_id [attr . id . 0] . r#match . is_none ()) . map (| attr | (attr . id , attr . inner . clone () , Some (id))) ,) ; } } false } }
    };
}

impl_36!();