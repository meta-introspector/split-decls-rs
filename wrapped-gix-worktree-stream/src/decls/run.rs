macro_rules! deps {
    () => {
        Delegate!();
        Source!();
        AdditionalEntry!();
        SharedErrorSlot!();
        Error!();
    };
}

macro_rules! run {
    () => {
        deps!();
        fn run < Find , E > (tree : gix_hash :: ObjectId , objects : Find , mut pipeline : gix_filter :: Pipeline , mut attributes : impl FnMut (& BStr , gix_object :: tree :: EntryMode , & mut gix_attributes :: search :: Outcome) -> Result < () , E > + Send + 'static , out : & mut gix_features :: io :: pipe :: Writer , err : SharedErrorSlot , additional_entries : std :: sync :: mpsc :: Receiver < AdditionalEntry > ,) -> Result < () , Error > where Find : gix_object :: Find + Clone , E : std :: error :: Error + Send + Sync + 'static , { let mut buf = Vec :: new () ; let tree_iter = objects . find_tree_iter (tree . as_ref () , & mut buf) ? ; if pipeline . driver_context_mut () . treeish . is_none () { pipeline . driver_context_mut () . treeish = Some (tree) ; } let mut attrs = gix_attributes :: search :: Outcome :: default () ; attrs . initialize_with_selection (& Default :: default () , Some ("export-ignore")) ; let mut dlg = traverse :: Delegate { out , err , pipeline , attrs , objects : objects . clone () , fetch_attributes : move | a : & BStr , b : gix_object :: tree :: EntryMode , c : & mut gix_attributes :: search :: Outcome | { attributes (a , b , c) . map_err (| err | Error :: Attributes { source : Box :: new (err) , path : a . to_owned () , }) } , path_deque : Default :: default () , path : Default :: default () , buf : Vec :: with_capacity (1024) , } ; gix_traverse :: tree :: breadthfirst (tree_iter , gix_traverse :: tree :: breadthfirst :: State :: default () , & objects , & mut dlg ,) ? ; for entry in additional_entries { protocol :: write_entry_header_and_path (entry . relative_path . as_ref () , & entry . id , entry . mode , entry . source . len () , out ,) ? ; # [allow (clippy :: unused_io_amount)] match entry . source { entry :: Source :: Memory (buf) => out . write (& buf) . map (| _ | ()) , entry :: Source :: Null => out . write (& []) . map (| _ | ()) , entry :: Source :: Path (path) => { let file = std :: fs :: File :: open (path) ? ; protocol :: write_stream (& mut buf , file , out) } } ? ; } Ok (()) }
    };
}

run!();