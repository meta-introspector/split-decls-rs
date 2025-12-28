macro_rules! namespace_starts_with {
    () => {
        fn namespace_starts_with (namespace : & str , starts_with : & str) -> bool { namespace . starts_with (starts_with) && (namespace . len () == starts_with . len () || namespace . as_bytes () . get (starts_with . len ()) == Some (& b'.')) }
    };
}

namespace_starts_with!()