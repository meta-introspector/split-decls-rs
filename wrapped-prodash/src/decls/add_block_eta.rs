macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! add_block_eta {
    () => {
        deps!();
        fn add_block_eta (state : progress :: State , progress_text : & mut String) { match state { progress :: State :: Blocked (reason , maybe_eta) | progress :: State :: Halted (reason , maybe_eta) => { progress_text . push_str (" [") ; progress_text . push_str (reason) ; progress_text . push (']') ; if let Some (eta) = maybe_eta { let eta = jiff :: Timestamp :: try_from (eta) . expect ("reasonable system time") ; let now = jiff :: Timestamp :: now () ; if eta > now { use std :: fmt :: Write ; write ! (progress_text , " → {:#} to {}" , eta . duration_since (now) , if let progress :: State :: Blocked (_ , _) = state { "unblock" } else { "continue" }) . expect ("in-memory writes never fail") ; } } } progress :: State :: Running => { } } }
    };
}

add_block_eta!()