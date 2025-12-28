macro_rules! deps {
    () => {
        PropertyUnicodeSet!();
        EmojiSet!();
        Baked!();
    };
}

macro_rules! make_emoji_set {
    () => {
        deps!();
        macro_rules ! make_emoji_set { (ident : $ ident : ident ; data_marker : $ data_marker : ty ; singleton : $ singleton : ident ; $ (# [$ doc : meta]) +) => { $ (# [$ doc]) + # [derive (Debug)] # [non_exhaustive] pub struct $ ident ; impl crate :: private :: Sealed for $ ident { } impl EmojiSet for $ ident { type DataMarker = $ data_marker ; # [cfg (feature = "compiled_data")] const SINGLETON : &'static crate :: provider :: PropertyUnicodeSet <'static > = & crate :: provider :: Baked ::$ singleton ; } } }
    };
}

make_emoji_set!();