macro_rules! swap_tails {
    () => {
        pub (crate) fn swap_tails (effective_base_url : Option < & str > , base_url : & str , mut url : String) -> String { match effective_base_url { Some (effective_base) => { url . replace_range (.. base_url . len () , effective_base) ; url } None => url , } }
    };
}

swap_tails!()