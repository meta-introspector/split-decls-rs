macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! ImportObjectData {
    () => {
        deps!();
        # [doc = " The data following [`pe::ImportObjectHeader`]."] # [derive (Debug , Clone)] pub struct ImportObjectData < 'data > { symbol : ByteString < 'data > , dll : ByteString < 'data > , export : Option < ByteString < 'data > > , }
    };
}

ImportObjectData!();