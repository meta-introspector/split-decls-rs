macro_rules! deps {
    () => {
        LazyAttrTokenStream!();
        AttrVec!();
    };
}

macro_rules! AttrsTarget {
    () => {
        deps!();
        # [doc = " Stores the tokens for an attribute target, along"] # [doc = " with its attributes."] # [doc = ""] # [doc = " This is constructed during parsing when we need to capture"] # [doc = " tokens, for `cfg` and `cfg_attr` attributes."] # [doc = ""] # [doc = " For example, `#[cfg(FALSE)] struct Foo {}` would"] # [doc = " have an `attrs` field containing the `#[cfg(FALSE)]` attr,"] # [doc = " and a `tokens` field storing the (unparsed) tokens `struct Foo {}`"] # [doc = ""] # [doc = " The `cfg`/`cfg_attr` processing occurs in"] # [doc = " `StripUnconfigured::configure_tokens`."] # [derive (Clone , Debug , Encodable , Decodable)] pub struct AttrsTarget { # [doc = " Attributes, both outer and inner."] # [doc = " These are stored in the original order that they were parsed in."] pub attrs : AttrVec , # [doc = " The underlying tokens for the attribute target that `attrs`"] # [doc = " are applied to"] pub tokens : LazyAttrTokenStream , }
    };
}

AttrsTarget!()