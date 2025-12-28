macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! Inserts {
    () => {
        deps!();
        # [doc = " Wrapper to [`UpdateVTab::insert`] arguments"] pub struct Inserts < 'a > { values : Values < 'a > , }
    };
}

Inserts!();