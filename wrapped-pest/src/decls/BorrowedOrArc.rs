macro_rules! deps {
    () => {
        SpanOrLiteral!();
    };
}

macro_rules! BorrowedOrArc {
    () => {
        deps!();
        # [doc = " A helper that provides efficient string handling without unnecessary copying."] # [doc = " We use `Arc<String>` instead of `Cow<'i, str>` to avoid copying strings when cloning the `Owned` variant, since"] # [doc = " `Arc::clone` only increments a reference count. [SpanOrLiteral] needs to be [Send] and [Sync]`, so we use [Arc]"] # [doc = " instead of [Rc]."] # [doc = ""] # [doc = " (We need to clone this struct to detach it from the `&self` borrow in [SpanOrLiteral::as_borrowed_or_rc], so that"] # [doc = " we can then call `self.match_string` (a `mut self` method)."] # [derive (Debug , Clone)] enum BorrowedOrArc < 'i > { Borrowed (& 'i str) , Owned (Arc < String >) , }
    };
}

BorrowedOrArc!();