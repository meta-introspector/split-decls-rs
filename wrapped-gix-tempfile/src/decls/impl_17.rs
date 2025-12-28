macro_rules! deps {
    () => {
        Handle!();
        Writable!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        # [doc = " Mutation"] impl Handle < Writable > { # [doc = " Obtain a mutable handler to the underlying named tempfile and call `f(&mut named_tempfile)` on it."] # [doc = ""] # [doc = " Note that for the duration of the call, a signal interrupting the operation will cause the tempfile not to be cleaned up"] # [doc = " as it is not visible anymore to the signal handler."] # [doc = ""] # [doc = " # Assumptions"] # [doc = " The caller must assure that the signal handler for cleanup will be followed by an abort call so that"] # [doc = " this code won't run again on a removed instance. An error will occur otherwise."] pub fn with_mut < T > (& mut self , once : impl FnOnce (& mut NamedTempFile) -> T) -> std :: io :: Result < T > { match REGISTRY . remove (& self . id) { Some ((id , Some (mut t))) => { let res = once (t . as_mut_tempfile () . expect ("correct runtime typing")) ; expect_none (REGISTRY . insert (id , Some (t))) ; Ok (res) } None | Some ((_ , None)) => Err (std :: io :: Error :: new (std :: io :: ErrorKind :: NotFound , format ! ("The tempfile with id {} wasn't available anymore" , self . id) ,)) , } } }
    };
}

impl_17!()