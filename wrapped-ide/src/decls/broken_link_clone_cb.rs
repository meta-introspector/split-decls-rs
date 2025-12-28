macro_rules! broken_link_clone_cb {
    () => {
        fn broken_link_clone_cb (link : BrokenLink < '_ >) -> Option < (CowStr < '_ > , CowStr < '_ >) > { Some ((link . reference . clone () , link . reference)) }
    };
}

broken_link_clone_cb!()