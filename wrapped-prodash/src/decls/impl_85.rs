macro_rules! deps {
    () => {
        JoinHandle!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl Drop for JoinHandle { fn drop (& mut self) { self . shutdown () ; self . inner . take () . and_then (| h | h . join () . ok ()) ; } }
    };
}

impl_85!();