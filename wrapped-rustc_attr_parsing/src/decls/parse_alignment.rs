macro_rules! parse_alignment {
    () => {
        fn parse_alignment (node : & LitKind) -> Result < Align , & 'static str > { if let LitKind :: Int (literal , LitIntType :: Unsuffixed) = node { if literal . get () . is_power_of_two () { literal . get () . try_into () . ok () . and_then (| v | Align :: from_bytes (v) . ok ()) . ok_or ("larger than 2^29") } else { Err ("not a power of two") } } else { Err ("not an unsuffixed integer") } }
    };
}

parse_alignment!();