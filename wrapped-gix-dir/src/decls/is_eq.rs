macro_rules! is_eq {
    () => {
        fn is_eq (lhs : & BStr , rhs : impl AsRef < BStr > , ignore_case : bool) -> bool { if ignore_case { lhs . eq_ignore_ascii_case (rhs . as_ref () . as_ref ()) } else { lhs == rhs . as_ref () } }
    };
}

is_eq!();