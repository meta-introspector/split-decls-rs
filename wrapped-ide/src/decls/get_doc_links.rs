macro_rules! deps {
    () => {
        DocumentationLinks!();
    };
}

macro_rules! get_doc_links {
    () => {
        deps!();
        fn get_doc_links (db : & RootDatabase , def : Definition , target_dir : Option < & str > , sysroot : Option < & str > ,) -> DocumentationLinks { let join_url = | base_url : Option < Url > , path : & str | -> Option < Url > { base_url . and_then (| url | url . join (path) . ok ()) } ; let Some ((target , file , frag)) = filename_and_frag_for_def (db , def) else { return Default :: default () ; } ; let (mut web_url , mut local_url) = get_doc_base_urls (db , target , target_dir , sysroot) ; let append_mod = ! matches ! (def , Definition :: Macro (m) if m . is_macro_export (db)) ; if append_mod && let Some (path) = mod_path_of_def (db , target) { web_url = join_url (web_url , & path) ; local_url = join_url (local_url , & path) ; } web_url = join_url (web_url , & file) ; local_url = join_url (local_url , & file) ; if let Some (url) = web_url . as_mut () { url . set_fragment (frag . as_deref ()) } if let Some (url) = local_url . as_mut () { url . set_fragment (frag . as_deref ()) } DocumentationLinks { web_url : web_url . map (| it | it . into ()) , local_url : local_url . map (| it | it . into ()) , } }
    };
}

get_doc_links!();