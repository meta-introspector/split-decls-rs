macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! ImportDescriptorIterator {
    () => {
        deps!();
        # [doc = " A fallible iterator for the descriptors in the import data directory."] # [derive (Debug , Clone)] pub struct ImportDescriptorIterator < 'data > { data : Bytes < 'data > , null : bool , }
    };
}

ImportDescriptorIterator!()