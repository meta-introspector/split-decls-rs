macro_rules! Format {
    () => {
        # [doc = " Format all iterator elements lazily, separated by `sep`."] # [doc = ""] # [doc = " The format value can only be formatted once, after that the iterator is"] # [doc = " exhausted."] # [doc = ""] # [doc = " See [`.format()`](../trait.Itertools.html#method.format)"] # [doc = " for more information."] # [derive (Clone)] pub struct Format < 'a , I > { sep : & 'a str , # [doc = " Format uses interior mutability because Display::fmt takes &self."] inner : RefCell < Option < I > > , }
    };
}

Format!();