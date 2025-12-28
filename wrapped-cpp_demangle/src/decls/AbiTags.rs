macro_rules! deps {
    () => {
        AbiTag!();
    };
}

macro_rules! AbiTags {
    () => {
        deps!();
        # [doc = " The `<abi-tags>` non-terminal."] # [doc = ""] # [doc = " ```text"] # [doc = " <abi-tags> ::= <abi-tag> [<abi-tags>]"] # [doc = " ```"] # [doc = ""] # [doc = " To make things easier on ourselves, despite the fact that the `<abi-tags>`"] # [doc = " production requires at least one tag, we'll allow a zero-length vector"] # [doc = " here instead of having to use Option<AbiTags> in everything that accepts"] # [doc = " an AbiTags."] # [derive (Clone , Debug , Default , PartialEq , Eq)] pub struct AbiTags (Vec < AbiTag >) ;
    };
}

AbiTags!()