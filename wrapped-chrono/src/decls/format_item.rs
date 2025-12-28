macro_rules! deps {
    () => {
        DelayedFormat!();
        NaiveTime!();
        Item!();
        FixedOffset!();
        NaiveDate!();
    };
}

macro_rules! format_item {
    () => {
        deps!();
        # [doc = " Formats single formatting item"] # [cfg (feature = "alloc")] # [deprecated (since = "0.4.32" , note = "Use DelayedFormat::fmt or DelayedFormat::write_to instead")] pub fn format_item (w : & mut fmt :: Formatter , date : Option < & NaiveDate > , time : Option < & NaiveTime > , off : Option < & (String , FixedOffset) > , item : & Item < '_ > ,) -> fmt :: Result { DelayedFormat { date : date . copied () , time : time . copied () , off : off . cloned () , items : [item] . into_iter () , locale : default_locale () , } . fmt (w) }
    };
}

format_item!();