macro_rules! DROPLESS_ALIGNMENT {
    () => {
        const DROPLESS_ALIGNMENT : usize = align_of :: < usize > () ;
    };
}

DROPLESS_ALIGNMENT!()