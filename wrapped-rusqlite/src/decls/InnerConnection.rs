macro_rules! deps {
    () => {
        Action!();
        BoxedAuthorizer!();
        PreUpdateCase!();
    };
}

macro_rules! InnerConnection {
    () => {
        deps!();
        pub struct InnerConnection { pub db : * mut ffi :: sqlite3 , interrupt_lock : Arc < Mutex < * mut ffi :: sqlite3 > > , # [cfg (feature = "hooks")] pub commit_hook : Option < Box < dyn FnMut () -> bool + Send > > , # [cfg (feature = "hooks")] pub rollback_hook : Option < Box < dyn FnMut () + Send > > , # [cfg (feature = "hooks")] # [expect (clippy :: type_complexity)] pub update_hook : Option < Box < dyn FnMut (crate :: hooks :: Action , & str , & str , i64) + Send > > , # [cfg (feature = "hooks")] pub progress_handler : Option < Box < dyn FnMut () -> bool + Send > > , # [cfg (feature = "hooks")] pub authorizer : Option < crate :: hooks :: BoxedAuthorizer > , # [cfg (feature = "preupdate_hook")] # [expect (clippy :: type_complexity)] pub preupdate_hook : Option < Box < dyn FnMut (crate :: hooks :: Action , & str , & str , & crate :: hooks :: PreUpdateCase) + Send > , > , owned : bool , }
    };
}

InnerConnection!();