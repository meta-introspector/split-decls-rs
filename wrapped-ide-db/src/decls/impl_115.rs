macro_rules! deps {
    () => {
        IsEmpty!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        # [allow (clippy :: disallowed_types)] impl < K , V , S > IsEmpty for std :: collections :: HashMap < K , V , S > { fn is_empty (& self) -> bool { self . is_empty () } }
    };
}

impl_115!();