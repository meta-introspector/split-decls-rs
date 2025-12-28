macro_rules! remove_links {
    () => {
        # [doc = " Remove all links in markdown documentation."] pub (crate) fn remove_links (markdown : & str) -> String { let mut drop_link = false ; let mut cb = | _ : BrokenLink < '_ > | { let empty = InlineStr :: try_from ("") . unwrap () ; Some ((CowStr :: Inlined (empty) , CowStr :: Inlined (empty))) } ; let doc = Parser :: new_with_broken_link_callback (markdown , MARKDOWN_OPTIONS , Some (& mut cb)) ; let doc = doc . filter_map (move | evt | match evt { Event :: Start (Tag :: Link (link_type , target , title)) => { if link_type == LinkType :: Inline && target . contains ("://") { Some (Event :: Start (Tag :: Link (link_type , target , title))) } else { drop_link = true ; None } } Event :: End (_) if drop_link => { drop_link = false ; None } _ => Some (evt) , }) ; let mut out = String :: new () ; cmark_resume_with_options (doc , & mut out , None , CMarkOptions { code_block_token_count : 3 , .. Default :: default () } ,) . ok () ; out }
    };
}

remove_links!();