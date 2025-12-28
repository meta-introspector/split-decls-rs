macro_rules! deps {
    () => {
        ChangeAnnotationId!();
        Indel!();
        UpmapFromRaFixture!();
    };
}

macro_rules! TextEdit {
    () => {
        deps!();
        # [derive (Default , Debug , Clone , UpmapFromRaFixture)] pub struct TextEdit { # [doc = " Invariant: disjoint and sorted by `delete`."] indels : Vec < Indel > , annotation : Option < ChangeAnnotationId > , }
    };
}

TextEdit!();