macro_rules! deps {
    () => {
        CustomFormat!();
    };
}

macro_rules! Format {
    () => {
        deps!();
        # [doc = " Various ways to describe a time format."] # [derive (Debug , Clone , Copy)] pub enum Format { # [doc = " A custom format limited to what's in the [`format`](mod@format) submodule."] Custom (CustomFormat) , # [doc = " The seconds since 1970, also known as unix epoch, like `1660874655`."] Unix , # [doc = " The seconds since 1970, followed by the offset, like `1660874655 +0800`"] Raw , }
    };
}

Format!();