macro_rules! tune_cpu {
    () => {
        pub (crate) fn tune_cpu (sess : & Session) -> Option < & str > { let name = sess . opts . unstable_opts . tune_cpu . as_ref () ? ; Some (handle_native (name)) }
    };
}

tune_cpu!();