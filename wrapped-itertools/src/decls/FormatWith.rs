macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! FormatWith {
    () => {
        deps!();
        # [doc = " Format all iterator elements lazily, separated by `sep`."] # [doc = ""] # [doc = " The format value can only be formatted once, after that the iterator is"] # [doc = " exhausted."] # [doc = ""] # [doc = " See [`.format_with()`](crate::Itertools::format_with) for more information."] pub struct FormatWith < 'a , I , F > { sep : & 'a str , # [doc = " `FormatWith` uses interior mutability because `Display::fmt` takes `&self`."] inner : Cell < Option < (I , F) > > , }
    };
}

FormatWith!();