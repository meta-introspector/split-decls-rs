macro_rules! change {
    () => {
        pub fn change (pct : f64 , signed : bool) -> String { if signed { format ! ("{:>+6}%" , signed_short (pct * 1e2)) } else { format ! ("{:>6}%" , short (pct * 1e2)) } }
    };
}

change!()