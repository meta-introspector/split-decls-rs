macro_rules! Grid {
    () => {
        # [doc = " Grid line"] # [derive (Clone , Copy)] pub enum Grid { # [doc = " Major gridlines"] Major , # [doc = " Minor gridlines"] Minor , }
    };
}

Grid!();