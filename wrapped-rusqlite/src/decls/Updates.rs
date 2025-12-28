macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! Updates {
    () => {
        deps!();
        # [doc = " Wrapper to [`UpdateVTab::update`] arguments"] pub struct Updates < 'a > { values : Values < 'a > , }
    };
}

Updates!();