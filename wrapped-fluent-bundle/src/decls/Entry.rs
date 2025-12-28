macro_rules! deps {
    () => {
        EntryIdx!();
        ResourceIdx!();
        FluentFunction!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " The [`Entry`] stores indexes into the [`FluentBundle`]'s resources for Messages and Terms,"] # [doc = " and owns the [`Box`] pointers to the [`FluentFunction`]."] pub enum Entry { Message ((ResourceIdx , EntryIdx)) , Term ((ResourceIdx , EntryIdx)) , Function (FluentFunction) , }
    };
}

Entry!()