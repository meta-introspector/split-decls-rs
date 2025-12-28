macro_rules! try_call_iter {
    () => {
        macro_rules ! try_call_iter { ($ ($ f : tt) *) => { match call ! ($ ($ f) *) { 0 => { } raw :: GIT_ITEROVER => return None , e => return Some (Err (crate :: call :: last_error (e))) } } }
    };
}

try_call_iter!()