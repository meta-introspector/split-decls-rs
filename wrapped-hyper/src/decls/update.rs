macro_rules! update {
    () => {
        # [cfg (feature = "http1")] pub (crate) fn update () { CACHED . with (| cache | { cache . borrow_mut () . check () ; }) }
    };
}

update!();