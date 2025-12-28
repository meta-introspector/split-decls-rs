macro_rules! deps {
    () => {
        Element!();
        Group!();
        Origin!();
    };
}

macro_rules! Snippet {
    () => {
        deps!();
        # [doc = " A source view [`Element`] in a [`Group`]"] # [doc = ""] # [doc = " If you do not have [source][Snippet::source] available, see instead [`Origin`]"] # [doc = ""] # [doc = " `Snippet`s come in the following styles (`T`):"] # [doc = " - With [`Annotation`]s, see [`Snippet::annotation`]"] # [doc = " - With [`Patch`]s, see [`Snippet::patch`]"] # [derive (Clone , Debug)] pub struct Snippet < 'a , T > { pub (crate) path : Option < Cow < 'a , str > > , pub (crate) line_start : usize , pub (crate) source : Cow < 'a , str > , pub (crate) markers : Vec < T > , pub (crate) fold : bool , }
    };
}

Snippet!()