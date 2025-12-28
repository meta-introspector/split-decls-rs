macro_rules! deps {
    () => {
        Group!();
    };
}

macro_rules! Report {
    () => {
        deps!();
        # [doc = " A [diagnostic message][Title] and any associated [context][Element] to help users"] # [doc = " understand it"] # [doc = ""] # [doc = " The first [`Group`] is the [\"primary\" group][Level::primary_title], ie it contains the diagnostic"] # [doc = " message."] # [doc = ""] # [doc = " All subsequent [`Group`]s are for distinct pieces of [context][Level::secondary_title]."] # [doc = " The primary group will be visually distinguished to help tell them apart."] pub type Report < 'a > = & 'a [Group < 'a >] ;
    };
}

Report!();