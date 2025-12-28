macro_rules! should_ignore_message {
    () => {
        fn should_ignore_message (i : & ast :: Item) -> Option < Symbol > { match attr :: find_by_name (& i . attrs , sym :: ignore) { Some (attr) => { match attr . meta_item_list () { Some (_) => None , None => attr . value_str () , } } None => None , } }
    };
}

should_ignore_message!();