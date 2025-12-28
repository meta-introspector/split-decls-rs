macro_rules! OptionCow {
    () => {
        # [derive (Debug)] pub struct OptionCow < 'a > (pub (crate) Option < Cow < 'a , str > >) ;
    };
}

OptionCow!()