macro_rules! deps {
    () => {
        ByteString!();
        AttributesSubsection!();
    };
}

macro_rules! impl_1171 {
    () => {
        deps!();
        impl < 'data > AttributesSubsection < 'data > { # [doc = " Create a new subsection."] pub fn new (vendor : ByteString < 'data >) -> Self { AttributesSubsection { vendor , subsubsections : Vec :: new () , } } }
    };
}

impl_1171!()