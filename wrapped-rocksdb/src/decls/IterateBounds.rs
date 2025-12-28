macro_rules! IterateBounds {
    () => {
        # [doc = " A range which can be set as iterate bounds on [`crate::ReadOptions`]."] # [doc = ""] # [doc = " See [`crate::ReadOptions::set_iterate_range`] for documentation and"] # [doc = " examples."] pub trait IterateBounds { # [doc = " Converts object into lower and upper bounds pair."] # [doc = ""] # [doc = " If this object represents range with one of the bounds unset,"] # [doc = " corresponding element is returned as `None`.  For example, `..upper`"] # [doc = " range would be converted into `(None, Some(upper))` pair."] fn into_bounds (self) -> (Option < Vec < u8 > > , Option < Vec < u8 > >) ; }
    };
}

IterateBounds!()