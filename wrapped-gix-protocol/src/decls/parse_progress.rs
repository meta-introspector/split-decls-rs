macro_rules! deps {
    () => {
        RemoteProgress!();
    };
}

macro_rules! parse_progress {
    () => {
        deps!();
        fn parse_progress < 'i > (line : & mut & 'i [u8]) -> ModalResult < RemoteProgress < 'i > , () > { let action = take_till (1 .. , | c | c == b':') . parse_next (line) ? ; let percent = next_optional_percentage . parse_next (line) ? ; let step = next_optional_number . parse_next (line) ? ; let max = next_optional_number . parse_next (line) ? ; Ok (RemoteProgress { action : action . into () , percent , step , max , }) }
    };
}

parse_progress!();