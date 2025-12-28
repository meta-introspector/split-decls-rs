macro_rules! deps {
    () => {
        Values!();
    };
}

macro_rules! Filters {
    () => {
        deps!();
        # [doc = " Wrapper to [`VTabCursor::filter`] arguments, the values"] # [doc = " requested by [`VTab::best_index`]."] pub struct Filters < 'a > { values : Values < 'a > , }
    };
}

Filters!()