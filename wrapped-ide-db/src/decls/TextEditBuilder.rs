macro_rules! deps {
    () => {
        ChangeAnnotationId!();
        Indel!();
    };
}

macro_rules! TextEditBuilder {
    () => {
        deps!();
        # [derive (Debug , Default , Clone)] pub struct TextEditBuilder { indels : Vec < Indel > , annotation : Option < ChangeAnnotationId > , }
    };
}

TextEditBuilder!()