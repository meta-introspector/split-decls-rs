macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Repository { pub fn root (& self) -> & Path { self . 0 . workdir () . unwrap () } pub fn url (& self) -> Url { self . 0 . workdir () . unwrap () . to_url () } pub fn revparse_head (& self) -> String { self . 0 . revparse_single ("HEAD") . expect ("revparse HEAD") . id () . to_string () } }
    };
}

impl_53!()