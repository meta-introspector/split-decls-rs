macro_rules! rewrite_url_link {
    () => {
        # [doc = " Try to resolve path to local documentation via path-based links (i.e. `../gateway/struct.Shard.html`)."] fn rewrite_url_link (db : & RootDatabase , def : Definition , target : & str) -> Option < String > { if ! (target . contains ('#') || target . contains (".html")) { return None ; } let mut url = get_doc_base_urls (db , def , None , None) . 0 ? ; let (def , file , frag) = filename_and_frag_for_def (db , def) ? ; if let Some (path) = mod_path_of_def (db , def) { url = url . join (& path) . ok () ? ; } url = url . join (& file) . ok () ? ; url . set_fragment (frag . as_deref ()) ; url . join (target) . ok () . map (Into :: into) }
    };
}

rewrite_url_link!()